pub struct User {
    pub id: UserId,
    pub name: UserName,
}

pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for UserId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<UserId> for i32 {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl AsRef<i32> for UserId {
    fn as_ref(&self) -> &i32 {
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
