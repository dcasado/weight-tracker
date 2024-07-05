use sqlx::{Pool, Postgres};

use crate::{
    domain::user::{User, UserId, UserName},
    error::ApiError,
};

pub async fn insert_user(pool: &Pool<Postgres>, name: String) -> Result<(), ApiError> {
    let _ = sqlx::query!(r#"INSERT INTO users (name) VALUES ($1)"#, name)
        .execute(pool)
        .await
        .map_err(|_| ApiError::Unknown)?;

    Ok(())
}

pub async fn find_users(pool: &Pool<Postgres>) -> Result<Vec<User>, ApiError> {
    struct Row {
        id: i32,
        name: String,
    }

    let rows = sqlx::query_as!(Row, r#"SELECT * FROM users"#)
        .fetch_all(pool)
        .await
        .map_err(|_| ApiError::Unknown)?;

    rows.into_iter()
        .map(|r| {
            Ok(User {
                id: UserId::new(r.id),
                name: UserName::new(r.name),
            })
        })
        .collect()
}

pub async fn find_user(pool: &Pool<Postgres>, user_id: i32) -> Result<Option<User>, ApiError> {
    struct Row {
        id: i32,
        name: String,
    }

    let row = sqlx::query_as!(Row, r#"SELECT id, name FROM users where id = $1"#, user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| ApiError::Unknown)?;

    Ok(row.map(|r| User {
        id: UserId::new(r.id),
        name: UserName::new(r.name),
    }))
}

pub async fn delete_user(pool: &Pool<Postgres>, id: i32) -> Result<(), ApiError> {
    let result = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::Unknown)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(())
}
