use serde::Deserialize;

use crate::services::ReservationRequestService;

#[derive(Deserialize)]
pub struct ReservationRequest {
    name: String,
    email: String,
    room: String,
}

impl ReservationRequestService for ReservationRequest {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn email(&self) -> String {
        self.email.clone()
    }

    fn room(&self) -> String {
        self.room.clone()
    }
}
