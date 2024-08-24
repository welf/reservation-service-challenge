use serde::Serialize;

use crate::services::UserService;

#[derive(Serialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl UserService for User {
    fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn email(&self) -> String {
        self.email.clone()
    }
}
