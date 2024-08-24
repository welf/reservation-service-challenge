use super::reservation_message_service::ReservationMessageService;

pub trait SendReservationMessageService {
    type Message: ReservationMessageService + Clone;

    fn send_message(&self, message: Self::Message);

    fn send_reservation_cancelled_message(
        &self,
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) {
        self.send_message(Self::Message::reservation_cancelled_message(to, name, room));
    }

    fn send_reservation_created_message(
        &self,
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) {
        self.send_message(Self::Message::reservation_created_message(to, name, room));
    }
}

pub trait GetOutboxService {
    type Message: ReservationMessageService + Clone;

    fn get_outbox(&self) -> Vec<Self::Message>;
}
