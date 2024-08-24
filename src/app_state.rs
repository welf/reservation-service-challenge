use std::sync::Arc;

use tokio::sync::Mutex;

use crate::services::{
    GetOutboxService, ReservationDatabaseService, SendReservationMessageService,
};

#[derive(Clone)]
pub struct AppState<DB, MessageService>
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
{
    pub db: Arc<Mutex<DB>>,
    pub mailer: MessageService,
}

impl<DB, MessageService> AppState<DB, MessageService>
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
{
    pub fn new(db: DB, mailer: MessageService) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            mailer,
        }
    }
}
