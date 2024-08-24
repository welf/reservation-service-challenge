pub trait ReservationMessageService {
    fn reservation_created_message(
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) -> Self;

    fn reservation_cancelled_message(
        to: impl Into<String>,
        name: impl Into<String>,
        room: impl Into<String>,
    ) -> Self;
}
