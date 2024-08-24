use serde::{Deserialize, Serialize};

use super::{ReservationService, UserService};

pub trait ReservationDatabaseService {
    type Reservation: ReservationService + Clone + for<'a> Serialize + for<'a> Deserialize<'a>;
    type User: UserService + Clone + for<'a> Serialize;

    fn get_reservations(&self) -> Vec<Self::Reservation>;
    fn get_reservation(&self, id: u64) -> Option<&Self::Reservation>;
    fn save_reservation(&mut self, reservation: Self::Reservation);
    fn delete_reservation(&mut self, id: u64);
    fn get_user(&self, email: String) -> Option<&Self::User>;
    fn save_user(&mut self, user: Self::User);
}
