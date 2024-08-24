use std::sync::{Arc, Mutex};

use crate::services::{GetOutboxService, ReservationMessageService, SendReservationMessageService};

#[derive(Clone)]
pub struct MockMailer<Message: ReservationMessageService + Clone> {
    emails: Arc<Mutex<Vec<Message>>>,
}

impl<Message: ReservationMessageService + Clone> MockMailer<Message> {
    pub fn new() -> Self {
        Self {
            emails: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<Message: ReservationMessageService + Clone> SendReservationMessageService
    for MockMailer<Message>
{
    type Message = Message;

    fn send_message(&self, message: Message) {
        let mut emails = self.emails.lock().unwrap();
        emails.push(message);
    }
}

impl<Message: ReservationMessageService + Clone> GetOutboxService for MockMailer<Message> {
    type Message = Message;

    fn get_outbox(&self) -> Vec<Message> {
        self.emails.lock().unwrap().clone()
    }
}
