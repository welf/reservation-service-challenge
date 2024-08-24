pub trait ReservationService {
    fn new(id: u64, email: impl Into<String>, room: impl Into<String>) -> Self;
    #[allow(dead_code)]
    fn id(&self) -> u64;
    fn email(&self) -> String;
    fn room(&self) -> String;
}
