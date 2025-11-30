use chrono::{DateTime, FixedOffset};
use sqlx::{Pool, Sqlite};

use crate::{
    domain::{
        impedance::{ImpedanceId, Ohms},
        user::UserId,
        weight::{Kilograms, Weight, WeightId},
    },
    error::ApiError,
};

pub async fn insert_weight(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    measured_at: &DateTime<FixedOffset>,
    kilograms: &Kilograms,
) -> Result<(), ApiError> {
    let user_id: i64 = user_id.into();
    let kilograms: f64 = kilograms.into();

    let _ = sqlx::query!(
        r#"INSERT INTO weight (user_id, measured_at, kilograms) VALUES ($1, $2, $3)"#,
        user_id,
        measured_at,
        kilograms
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(())
}

pub async fn insert_impedance(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    measured_at: &DateTime<FixedOffset>,
    ohms: &Ohms,
) -> Result<(), ApiError> {
    let user_id: i64 = user_id.into();
    let ohms: f64 = ohms.into();

    let _ = sqlx::query!(
        r#"INSERT INTO impedance (user_id, measured_at, ohms) VALUES ($1, $2, $3)"#,
        user_id,
        measured_at,
        ohms
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(())
}

pub async fn find_weights_by_year_month(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    year: &str,
    month: &str,
) -> Result<Vec<Weight>, ApiError> {
    struct Row {
        weight_id: i64,
        user_id: i64,
        measured_at: String,
        kilograms: f64,
    }

    let user_id: i64 = user_id.into();

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT weight_id, user_id, measured_at, kilograms FROM weight WHERE user_id = $1 AND strftime('%Y', measured_at) = $2 AND strftime('%m', measured_at) = $3 ORDER BY measured_at DESC"#,
        user_id,
        year,
        month
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(Weight {
                weight_id: WeightId::new(r.weight_id),
                user_id: UserId::new(r.user_id),
                measured_at: DateTime::parse_from_rfc3339(r.measured_at.as_str())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?,
                kilograms: Kilograms::new(r.kilograms)?,
            })
        })
        .collect()
}

pub async fn find_weights_between_dates(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    start_date: &DateTime<FixedOffset>,
    end_date: &DateTime<FixedOffset>,
) -> Result<Vec<Weight>, ApiError> {
    struct Row {
        weight_id: i64,
        user_id: i64,
        measured_at: String,
        kilograms: f64,
    }

    let user_id: i64 = user_id.into();

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT weight_id, user_id, measured_at, kilograms FROM weight WHERE user_id = $1 AND measured_at BETWEEN $2 AND $3 ORDER BY measured_at ASC"#,
        user_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    rows.into_iter()
        .map(|r| {
            Ok(Weight {
                weight_id: WeightId::new(r.weight_id),
                user_id: UserId::new(r.user_id),
                measured_at: DateTime::parse_from_rfc3339(r.measured_at.as_str())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?,
                kilograms: Kilograms::new(r.kilograms)?,
            })
        })
        .collect()
}

pub async fn find_duplicate_weights(
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
        r#"SELECT date(measured_at, 'localtime') AS date, COUNT(*) as counter FROM weight WHERE user_id = $1 GROUP BY date HAVING COUNT(*) > 1"#,
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

pub async fn find_years(pool: &Pool<Sqlite>, user_id: &UserId) -> Result<Vec<String>, ApiError> {
    struct Row {
        year: Option<String>,
    }

    let user_id: i64 = user_id.into();

    let year_result = sqlx::query_as!(
        Row,
        r#"SELECT DISTINCT strftime('%Y', measured_at) AS year FROM weight WHERE user_id = $1 ORDER BY year DESC"#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(year_result
        .into_iter()
        .map(|r| {
            r.year
                .expect("year returned from date on the database should not be none")
        })
        .collect())
}

pub async fn find_months_by_year(
    pool: &Pool<Sqlite>,
    user_id: &UserId,
    year: &str,
) -> Result<Vec<String>, ApiError> {
    struct Row {
        month: Option<String>,
    }

    let user_id: i64 = user_id.into();

    let month_result = sqlx::query_as!(
        Row,
        r#"SELECT DISTINCT strftime('%m', measured_at) AS month FROM weight WHERE user_id = $1 AND strftime('%Y', measured_at) = $2 ORDER BY month DESC"#,
        user_id,
        year
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(month_result
        .into_iter()
        .map(|r| {
            r.month
                .expect("month returned from date on the database should not be none")
        })
        .collect())
}

pub async fn delete_weight(pool: &Pool<Sqlite>, weight_id: &WeightId) -> Result<(), ApiError> {
    let weight_id: i64 = weight_id.into();

    let result = sqlx::query!(r#"DELETE FROM weight WHERE weight_id = $1"#, weight_id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::WeightNotFound);
    }

    Ok(())
}

pub async fn delete_impedance(
    pool: &Pool<Sqlite>,
    impedance_id: &ImpedanceId,
) -> Result<(), ApiError> {
    let impedance_id: i64 = impedance_id.into();

    let result = sqlx::query!(
        r#"DELETE FROM impedance WHERE impedance_id = $1"#,
        impedance_id
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::ImpedanceNotFound);
    }

    Ok(())
}
