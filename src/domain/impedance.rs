use chrono::{DateTime, FixedOffset};

use crate::error::ApiError;

use super::user::UserId;

pub struct Impedance {
    pub impedance_id: ImpedanceId,
    pub user_id: UserId,
    pub measured_at: DateTime<FixedOffset>,
    pub ohms: Ohms,
}

pub struct ImpedanceId(i64);

impl ImpedanceId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl From<ImpedanceId> for i64 {
    fn from(value: ImpedanceId) -> Self {
        value.0
    }
}

impl From<&ImpedanceId> for i64 {
    fn from(value: &ImpedanceId) -> Self {
        value.0
    }
}

#[derive(Clone)]
pub struct Ohms(f64);

impl Ohms {
    pub fn new(value: f64) -> Result<Ohms, ApiError> {
        if value < 0.0 {
            return Err(ApiError::NegativeWeight);
        }
        Ok(Ohms(value))
    }
}

impl From<Ohms> for f64 {
    fn from(value: Ohms) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Ohms {
    type Error = ApiError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<&Ohms> for f64 {
    fn from(value: &Ohms) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_impedance_is_invalid() -> Result<(), String> {
        let negative_impedance = -1.0;

        match Ohms::try_from(negative_impedance) {
            Ok(_) => Err("Impedance cannot be negative".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn positive_impedance_is_valid() -> Result<(), String> {
        let impedance = 1.0;

        match Ohms::try_from(impedance) {
            Ok(w) => {
                assert_eq!(w.0, 1.0, "Impedance does not match");
                Ok(())
            }
            Err(_) => Err("Impedance must be possitive".to_string()),
        }
    }
}
