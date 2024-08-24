use serde::{Deserialize, Serialize};

use crate::services::ReservationMessageService;

#[derive(Serialize, Deserialize, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl ReservationMessageService for Email {
    fn reservation_created_message(
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) -> Self {
        Self {
            to: to.into(),
            subject: "Reservation created".to_string(),
            body: format!(
                "Hello, {}. Your reservation for room {} has been created",
                name.into(),
                room.into()
            ),
        }
    }

    fn reservation_cancelled_message(
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) -> Self {
        Self {
            to: to.into(),
            subject: "Reservation cancelled".to_string(),
            body: format!(
                "Hello, {}. Your reservation for room {} has been cancelled",
                name.into(),
                room.into()
            ),
        }
    }
}
