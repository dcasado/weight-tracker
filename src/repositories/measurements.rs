use chrono::{DateTime, FixedOffset};
use sqlx::{Pool, Sqlite};

use crate::{
    domain::{
        measurement::{Measurement, MeasurementId, Weight},
        user::UserId,
    },
    error::ApiError,
};

pub async fn insert_measurement(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    date_time: &DateTime<FixedOffset>,
    weight: &Weight,
) -> Result<(), ApiError> {
    let user_id: i64 = user_id.into();
    let weight: f64 = weight.into();

    let _ = sqlx::query!(
        r#"INSERT INTO measurements (user_id, date_time, weight) VALUES ($1, $2, $3)"#,
        user_id,
        date_time,
        weight
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(())
}

pub async fn find_measurements(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
) -> Result<Vec<Measurement>, ApiError> {
    struct Row {
        id: i64,
        user_id: i64,
        date_time: String,
        weight: f64,
    }

    let user_id: i64 = user_id.into();

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, user_id, date_time, weight FROM measurements WHERE user_id = $1 ORDER BY date_time DESC"#,
        user_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(Measurement {
                id: MeasurementId::new(r.id),
                user_id: UserId::new(r.user_id),
                date_time: DateTime::parse_from_rfc3339(r.date_time.as_str())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?,
                weight: Weight::new(r.weight)?,
            })
        })
        .collect()
}

pub async fn find_measurements_between_dates(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    start_date: &DateTime<FixedOffset>,
    end_date: &DateTime<FixedOffset>,
) -> Result<Vec<Measurement>, ApiError> {
    struct Row {
        id: i64,
        user_id: i64,
        date_time: String,
        weight: f64,
    }

    let user_id: i64 = user_id.into();

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, user_id, date_time, weight FROM measurements WHERE user_id = $1 AND date_time BETWEEN $2 AND $3 ORDER BY date_time ASC"#,
        user_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(Measurement {
                id: MeasurementId::new(r.id),
                user_id: UserId::new(r.user_id),
                date_time: DateTime::parse_from_rfc3339(r.date_time.as_str())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?,
                weight: Weight::new(r.weight)?,
            })
        })
        .collect()
}

pub async fn find_duplicate_measurements(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
) -> Result<Vec<(String, i64)>, ApiError> {
    struct Row {
        date: Option<String>,
        counter: i64,
    }

    let user_id: i64 = user_id.into();
    let result = sqlx::query_as!(
        Row,
        r#"SELECT date(date_time, 'localtime') AS date, COUNT(*) as counter FROM measurements WHERE user_id = $1 GROUP BY date HAVING COUNT(*) > 1"#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    result
        .into_iter()
        .map(|r| {
            Ok((
                r.date
                    .expect("date returned from the database should not be none"),
                r.counter,
            ))
        })
        .collect()
}

pub async fn delete_measurement(pool: &Pool<Sqlite>, id: &MeasurementId) -> Result<(), ApiError> {
    let id: i64 = id.into();

    let result = sqlx::query!(r#"DELETE FROM measurements WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::MeasurementNotFound);
    }

    Ok(())
}
