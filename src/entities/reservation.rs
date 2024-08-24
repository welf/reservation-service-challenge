use serde::{Deserialize, Serialize};

use crate::services::ReservationService;

#[derive(Serialize, Deserialize, Clone)]
pub struct Reservation {
    pub id: u64,
    pub email: String,
    pub room: String,
}

impl ReservationService for Reservation {
    fn new(id: u64, email: impl Into<String>, room: impl Into<String>) -> Self {
        Self {
            id,
            email: email.into(),
            room: room.into(),
        }
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn email(&self) -> String {
        self.email.clone()
    }

    fn room(&self) -> String {
        self.room.clone()
    }
}
