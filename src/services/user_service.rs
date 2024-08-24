pub trait UserService {
    fn new(name: impl Into<String>, email: impl Into<String>) -> Self;
    fn name(&self) -> String;
    fn email(&self) -> String;
}
