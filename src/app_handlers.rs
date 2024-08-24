use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;

use crate::{
    app_state::AppState,
    services::{
        CreateReservationRequestService, GetOutboxService, ReservationDatabaseService,
        ReservationService, SendReservationMessageService, UserService,
    },
};

pub async fn get_reservations<DB, MessageService>(
    State(state): State<AppState<DB, MessageService>>,
) -> Json<Vec<DB::Reservation>>
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
{
    let db = state.db.lock().await;
    Json(db.get_reservations())
}

pub async fn create_reservation<DB, MessageService, ReservationRequest>(
    State(state): State<AppState<DB, MessageService>>,
    Json(payload): Json<ReservationRequest>,
) -> StatusCode
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
    ReservationRequest: CreateReservationRequestService,
{
    let mut db = state.db.lock().await;
    let id = db.get_reservations().len() as u64 + 1;

    if db.get_user(payload.email()).is_none() {
        let user = DB::User::new(payload.name(), payload.email());
        db.save_user(user);
    }

    let reservation = DB::Reservation::new(id, payload.email(), payload.room());

    db.save_reservation(reservation);

    state
        .mailer
        .send_reservation_created_message(payload.email(), payload.name(), payload.room());

    StatusCode::CREATED
}

pub async fn delete_reservation<DB, MessageService>(
    Path(id): Path<u64>,
    State(state): State<AppState<DB, MessageService>>,
) -> StatusCode
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
{
    let mut db = state.db.lock().await;
    if let Some(reservation) = db.get_reservation(id) {
        if let Some(user) = db.get_user(reservation.email()) {
            state.mailer.send_reservation_cancelled_message(
                user.email(),
                user.name(),
                reservation.room(),
            );
        }
        db.delete_reservation(id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_outbox<DB, MessageService>(
    State(state): State<AppState<DB, MessageService>>,
) -> Json<Vec<<MessageService as GetOutboxService>::Message>>
where
    DB: ReservationDatabaseService + Clone,
    MessageService: SendReservationMessageService + GetOutboxService,
{
    let outbox = state.mailer.get_outbox();
    Json(outbox)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        database::MockDatabase,
        drivers::MockMailer,
        entities::{CreateReservation, Reservation},
        messages::Email,
    };
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        routing::{delete, get, post},
        Router,
    };
    use serde_json::json;
    use tower::util::ServiceExt;

    async fn setup() -> Router {
        let db = MockDatabase::new();
        let mailer: MockMailer<Email> = MockMailer::new();
        let state = AppState::new(db, mailer);
        Router::new()
            .route("/reservations", get(get_reservations))
            .route(
                "/reservations",
                post(create_reservation::<MockDatabase, MockMailer<Email>, CreateReservation>),
            )
            .route("/reservations/:id", delete(delete_reservation))
            .route("/mailer/outbox", get(get_outbox))
            .with_state(state)
    }
    #[tokio::test]
    async fn test_reservations() {
        let app = setup().await;
        let app_clone = app.clone();
        let app_clone2 = app.clone();

        let payload = json!({
            "name": "John Doe",
            "email": "john@example.com",
            "room": "101"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/reservations")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = Request::builder()
            .method("GET")
            .uri("/reservations")
            .body(Body::empty())
            .unwrap();

        let response = app_clone.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let reservations: Vec<Reservation> = serde_json::from_slice(&body).unwrap();
        assert_eq!(reservations.len(), 1);
        assert_eq!(reservations[0].email, "john@example.com");
        assert_eq!(reservations[0].room, "101");

        let request = Request::builder()
            .method("DELETE")
            .uri("/reservations/1")
            .body(Body::empty())
            .unwrap();

        let response = app_clone2.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_delete_reservation_not_found() {
        let app = setup().await;

        let request = Request::builder()
            .method("DELETE")
            .uri("/reservations/999")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_outbox() {
        let app = setup().await;
        let app_clone = app.clone();
        let app_clone2 = app.clone();

        // Create a reservation
        let payload = json!({
            "name": "John Doe",
            "email": "john@example.com",
            "room": "101"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/reservations")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        // Delete the reservation
        let request = Request::builder()
            .method("DELETE")
            .uri("/reservations/1")
            .body(Body::empty())
            .unwrap();

        let response = app_clone.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let request = Request::builder()
            .method("GET")
            .uri("/mailer/outbox")
            .body(Body::empty())
            .unwrap();

        let response = app_clone2.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let emails: Vec<Email> = serde_json::from_slice(&body).unwrap();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].subject, "Reservation created");
        assert_eq!(emails[1].subject, "Reservation cancelled");
    }
}
