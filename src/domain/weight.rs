use chrono::{DateTime, FixedOffset};

use crate::error::ApiError;

use super::user::UserId;

pub struct Weight {
    pub weight_id: WeightId,
    pub user_id: UserId,
    pub measured_at: DateTime<FixedOffset>,
    pub kilograms: Kilograms,
}

pub struct WeightId(i64);

impl WeightId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl From<WeightId> for i64 {
    fn from(value: WeightId) -> Self {
        value.0
    }
}

impl From<&WeightId> for i64 {
    fn from(value: &WeightId) -> Self {
        value.0
    }
}

#[derive(Clone)]
pub struct Kilograms(f64);

impl Kilograms {
    pub fn new(value: f64) -> Result<Kilograms, ApiError> {
        if value < 0.0 {
            return Err(ApiError::NegativeWeight);
        }
        Ok(Kilograms(value))
    }
}

impl From<Kilograms> for f64 {
    fn from(value: Kilograms) -> Self {
        value.0
    }
}

impl From<&Kilograms> for f64 {
    fn from(value: &Kilograms) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Kilograms {
    type Error = ApiError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<f64> for Kilograms {
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

        match Kilograms::try_from(negative_weight) {
            Ok(_) => Err("Weight cannot be negative".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn positive_weight_is_valid() -> Result<(), String> {
        let weight = 0.1;

        match Kilograms::try_from(weight) {
            Ok(w) => {
                assert_eq!(w.0, 0.1, "Weight does not match");
                Ok(())
            }
            Err(_) => Err("Weight must be possitive".to_string()),
        }
    }
}
