use std::collections::HashMap;

use crate::{
    entities::{Reservation, User},
    services::ReservationDatabaseService,
};

#[derive(Clone)]
pub struct MockDatabase {
    reservations: HashMap<u64, Reservation>,
    users: HashMap<String, User>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self {
            reservations: HashMap::new(),
            users: HashMap::new(),
        }
    }
}

impl ReservationDatabaseService for MockDatabase {
    type Reservation = Reservation;
    type User = User;

    fn get_reservations(&self) -> Vec<Reservation> {
        self.reservations.values().cloned().collect()
    }

    fn get_reservation(&self, id: u64) -> Option<&Reservation> {
        self.reservations.get(&id)
    }

    fn save_reservation(&mut self, reservation: Reservation) {
        self.reservations.insert(reservation.id, reservation);
    }

    fn delete_reservation(&mut self, id: u64) {
        self.reservations.remove(&id);
    }

    fn get_user(&self, email: String) -> Option<&User> {
        self.users.get(&email)
    }

    fn save_user(&mut self, user: User) {
        self.users.insert(user.email.clone(), user);
    }
}
