use sqlx::{Pool, Sqlite};

use crate::{
    domain::user::{User, UserId, UserName},
    error::ApiError,
};

pub async fn insert_user(pool: &Pool<Sqlite>, name: &UserName) -> Result<(), ApiError> {
    let name: String = name.into();

    let _ = sqlx::query!(r#"INSERT INTO users (name) VALUES ($1)"#, name)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(())
}

pub async fn find_users(pool: &Pool<Sqlite>) -> Result<Vec<User>, ApiError> {
    struct Row {
        id: i64,
        name: String,
    }

    let rows = sqlx::query_as!(Row, r#"SELECT * FROM users"#)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(User {
                id: UserId::new(r.id),
                name: UserName::new(r.name),
            })
        })
        .collect()
}

pub async fn find_user(pool: &Pool<Sqlite>, user_id: &UserId) -> Result<Option<User>, ApiError> {
    struct Row {
        id: i64,
        name: String,
    }

    let user_id: i64 = user_id.into();

    let row = sqlx::query_as!(Row, r#"SELECT id, name FROM users where id = $1"#, user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(row.map(|r| User {
        id: UserId::new(r.id),
        name: UserName::new(r.name),
    }))
}

pub async fn delete_user(pool: &Pool<Sqlite>, id: &UserId) -> Result<(), ApiError> {
    let id: i64 = id.into();

    let result = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(())
}
