use chrono::{DateTime, FixedOffset};

use crate::error::ApiError;

use super::user::UserId;

pub struct Measurement {
    pub id: MeasurementId,
    pub user_id: UserId,
    pub date_time: DateTime<FixedOffset>,
    pub weight: Weight,
}

pub struct MeasurementId(i64);

impl MeasurementId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl From<MeasurementId> for i64 {
    fn from(value: MeasurementId) -> Self {
        value.0
    }
}

impl From<&MeasurementId> for i64 {
    fn from(value: &MeasurementId) -> Self {
        value.0
    }
}

pub struct Weight(f64);

impl Weight {
    pub fn new(weight: f64) -> Result<Weight, ApiError> {
        if weight < 0.0 {
            return Err(ApiError::NegativeWeight);
        }
        Ok(Weight(weight))
    }
}

impl From<Weight> for f64 {
    fn from(value: Weight) -> Self {
        value.0
    }
}

impl From<&Weight> for f64 {
    fn from(value: &Weight) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Weight {
    type Error = ApiError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<f64> for Weight {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_weight_is_invalid() -> Result<(), String> {
        let negative_weight = -0.1;

        match Weight::try_from(negative_weight) {
            Ok(_) => Err("Weight cannot be negative".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn positive_weight_is_valid() -> Result<(), String> {
        let weight = 0.1;

        match Weight::try_from(weight) {
            Ok(w) => {
                assert_eq!(w.0, 0.1, "Weight does not match");
                Ok(())
            }
            Err(_) => Err("Weight must be possitive".to_string()),
        }
    }
}
