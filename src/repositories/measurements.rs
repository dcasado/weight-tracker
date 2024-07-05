use chrono::{DateTime, FixedOffset};
use sqlx::{Pool, Postgres};

use crate::{
    domain::{
        measurement::{Measurement, MeasurementId, Weight},
        user::UserId,
    },
    error::ApiError,
};

pub async fn insert_measurement(
    pool: &Pool<Postgres>,
    user_id: &UserId,
    date_time: &DateTime<FixedOffset>,
    weight: &Weight,
) -> Result<(), ApiError> {
    let _ = sqlx::query!(
        r#"INSERT INTO measurements (user_id, date_time, weight) VALUES ($1, $2, $3)"#,
        user_id.as_ref(),
        date_time,
        weight.as_ref()
    )
    .execute(pool)
    .await
    .map_err(|_| ApiError::Unknown)?;

    Ok(())
}

pub async fn find_measurements(
    pool: &Pool<Postgres>,
    user_id: &i32,
) -> Result<Vec<Measurement>, ApiError> {
    struct Row {
        id: i32,
        user_id: i32,
        date_time: DateTime<FixedOffset>,
        weight: f64,
    }

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, user_id, date_time, weight FROM measurements WHERE user_id = $1 ORDER BY date_time ASC"#,
        user_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| ApiError::Unknown)?;

    rows.into_iter()
        .map(|r| {
            Ok(Measurement {
                id: MeasurementId::new(r.id),
                user_id: UserId::new(r.user_id),
                date_time: r.date_time,
                weight: Weight::new(r.weight).map_err(|_| ApiError::Unknown)?,
            })
        })
        .collect()
}

pub async fn find_measurements_between_dates(
    pool: &Pool<Postgres>,
    user_id: &i32,
    start_date: DateTime<FixedOffset>,
    end_date: DateTime<FixedOffset>,
) -> Result<Vec<Measurement>, ApiError> {
    struct Row {
        id: i32,
        user_id: i32,
        date_time: DateTime<FixedOffset>,
        weight: f64,
    }

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, user_id, date_time, weight FROM measurements WHERE user_id = $1 AND date_time BETWEEN $2 AND $3 ORDER BY date_time ASC"#,
        user_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
    .map_err(|_| ApiError::Unknown)?;

    rows.into_iter()
        .map(|r| {
            Ok(Measurement {
                id: MeasurementId::new(r.id),
                user_id: UserId::new(r.user_id),
                date_time: r.date_time,
                weight: Weight::new(r.weight).map_err(|_| ApiError::Unknown)?,
            })
        })
        .collect()
}

pub async fn delete_measurement(pool: &Pool<Postgres>, id: i32) -> Result<(), ApiError> {
    let result = sqlx::query!(r#"DELETE FROM measurements WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::Unknown)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::MeasurementNotFound);
    }

    Ok(())
}
