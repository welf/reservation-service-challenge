pub trait ReservationRequestService {
    fn name(&self) -> String;
    fn email(&self) -> String;
    fn room(&self) -> String;
}
