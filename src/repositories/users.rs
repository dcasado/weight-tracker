use sqlx::{Pool, Sqlite};

use crate::{
    domain::user::{BirthDate, Height, User, UserId, UserName},
    error::ApiError,
};

pub async fn insert_user(
    pool: &Pool<Sqlite>,
    name: &UserName,
    birthdate: &BirthDate,
    height: &Height,
) -> Result<(), ApiError> {
    let name: String = name.into();
    let birthdate: String = birthdate.into();
    let height: f64 = height.into();

    let _ = sqlx::query!(
        r#"INSERT INTO person (name, birthdate, height) VALUES ($1, $2, $3)"#,
        name,
        birthdate,
        height
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(())
}

pub async fn find_users(pool: &Pool<Sqlite>) -> Result<Vec<User>, ApiError> {
    struct Row {
        person_id: i64,
        name: String,
        birthdate: Option<String>,
        height: Option<f64>,
    }

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT person_id, name, birthdate, height FROM person"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(User {
                id: UserId::new(r.person_id),
                name: UserName::new(r.name),
            })
        })
        .collect()
}

pub async fn find_user(pool: &Pool<Sqlite>, user_id: &UserId) -> Result<Option<User>, ApiError> {
    struct Row {
        person_id: i64,
        name: String,
    }

    let user_id: i64 = user_id.into();

    let row = sqlx::query_as!(
        Row,
        r#"SELECT person_id, name FROM person where person_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(row.map(|r| User {
        id: UserId::new(r.person_id),
        name: UserName::new(r.name),
    }))
}

pub async fn delete_user(pool: &Pool<Sqlite>, person_id: &UserId) -> Result<(), ApiError> {
    let person_id: i64 = person_id.into();

    let result = sqlx::query!(r#"DELETE FROM person WHERE person_id = $1"#, person_id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(())
}
