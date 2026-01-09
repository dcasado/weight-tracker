use crate::error::ApiError;

pub struct User {
    pub id: UserId,
    pub name: UserName,
}

pub struct UserId(i64);

impl UserId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
}

impl From<i64> for UserId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UserId> for i64 {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl From<&UserId> for i64 {
    fn from(value: &UserId) -> Self {
        value.0
    }
}

impl AsRef<i64> for UserId {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}

impl From<&UserName> for String {
    fn from(value: &UserName) -> Self {
        value.0.clone()
    }
}

pub struct BirthDate(String);

impl BirthDate {
    pub fn new(value: &str) -> Result<Self, ApiError> {
        let date =
            chrono::DateTime::parse_from_rfc3339(value).map_err(|_| ApiError::InvalidBirthdate)?;
        Ok(Self(date.to_rfc3339()))
    }
}

impl From<&BirthDate> for String {
    fn from(value: &BirthDate) -> Self {
        value.0.clone()
    }
}

pub struct Height(u16);

impl Height {
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

impl From<&Height> for f64 {
    fn from(value: &Height) -> Self {
        value.0.into()
    }
}
