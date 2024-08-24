mod app_handlers;
mod app_state;
mod database;
mod drivers;
mod entities;
mod messages;
mod reservation;
mod services;

use app_handlers::{create_reservation, delete_reservation, get_outbox, get_reservations};
use app_state::AppState;
use database::MockDatabase;
use drivers::MockMailer;
use entities::CreateReservation;
use messages::Email;

use axum::{
    routing::{delete, get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let db = MockDatabase::new();
    let mailer: MockMailer<Email> = MockMailer::new();
    let state = AppState::new(db, mailer);

    let app = Router::new()
        .route("/reservations", get(get_reservations))
        .route(
            "/reservations",
            post(create_reservation::<MockDatabase, MockMailer<Email>, CreateReservation>),
        )
        .route("/reservations/:id", delete(delete_reservation))
        .route("/mailer/outbox", get(get_outbox))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
