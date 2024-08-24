use serde::Deserialize;

use crate::services::CreateReservationRequestService;

#[derive(Deserialize)]
pub struct CreateReservation {
    name: String,
    email: String,
    room: String,
}

impl CreateReservationRequestService for CreateReservation {
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
