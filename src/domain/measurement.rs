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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Impedance(i64);

impl Impedance {
    pub fn new(impedance: i64) -> Result<Impedance, ApiError> {
        if impedance < 0 {
            return Err(ApiError::NegativeWeight);
        }
        Ok(Impedance(impedance))
    }
}

impl TryFrom<i64> for Impedance {
    type Error = ApiError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<&Impedance> for i64 {
    fn from(value: &Impedance) -> Self {
        value.0
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

    #[test]
    fn negative_impedance_is_invalid() -> Result<(), String> {
        let negative_impedance = -1;

        match Impedance::try_from(negative_impedance) {
            Ok(_) => Err("Impedance cannot be negative".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn positive_impedance_is_valid() -> Result<(), String> {
        let impedance = 1;

        match Impedance::try_from(impedance) {
            Ok(w) => {
                assert_eq!(w.0, 1, "Impedance does not match");
                Ok(())
            }
            Err(_) => Err("Impedance must be possitive".to_string()),
        }
    }
}
