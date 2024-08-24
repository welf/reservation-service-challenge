use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct CreateReservation {
    name: String,
    email: String,
    room: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Reservation {
    id: u64,
    email: String,
    room: String,
}

#[derive(Serialize, Clone)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Email {
    to: String,
    subject: String,
    body: String,
}

struct Database {
    reservations: HashMap<u64, Reservation>,
    users: HashMap<String, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            reservations: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn get_reservation(&self, id: u64) -> Option<&Reservation> {
        self.reservations.get(&id)
    }

    fn save_reservation(&mut self, reservation: Reservation) {
        self.reservations.insert(reservation.id, reservation);
    }

    fn get_user(&self, email: String) -> Option<&User> {
        self.users.get(&email)
    }

    fn save_user(&mut self, user: User) {
        self.users.insert(user.email.clone(), user);
    }
}

#[derive(Clone)]
struct Mailer {
    emails: Arc<Mutex<Vec<Email>>>,
}

impl Mailer {
    fn new() -> Self {
        Self {
            emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn send_email(&self, email: Email) {
        let mut emails = self.emails.lock().unwrap();
        emails.push(email);
    }

    fn get_outbox(&self) -> Vec<Email> {
        let emails = self.emails.lock().unwrap();
        emails.clone()
    }
}

#[derive(Clone)]
struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub mailer: Mailer,
}

impl AppState {
    fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(Database::new())),
            mailer: Mailer::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/reservations", get(get_reservations))
        .route("/reservations", post(create_reservation))
        .route("/reservations/:id", delete(delete_reservation))
        .route("/mailer/outbox", get(get_outbox))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_reservations(State(state): State<AppState>) -> Json<Vec<Reservation>> {
    let db = state.db.lock().unwrap();
    Json(db.reservations.values().cloned().collect())
}

async fn create_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> StatusCode {
    let mut db = state.db.lock().unwrap();
    let id = db.reservations.len() as u64 + 1;

    if db.get_user(payload.email.clone()).is_none() {
        let user = User {
            name: payload.name.clone(),
            email: payload.email.clone(),
        };
        db.save_user(user);
    }

    let reservation = Reservation {
        id,
        email: payload.email.clone(),
        room: payload.room.clone(),
    };
    db.save_reservation(reservation);

    state.mailer.send_email(Email {
        to: payload.email,
        subject: "Reservation created".to_string(),
        body: format!(
            "Hello, {}. Your reservation for room {} has been created",
            payload.name, payload.room
        ),
    });
    StatusCode::CREATED
}

async fn delete_reservation(Path(id): Path<u64>, State(state): State<AppState>) -> StatusCode {
    let mut db = state.db.lock().unwrap();
    if let Some(reservation) = db.get_reservation(id) {
        if let Some(user) = db.get_user(reservation.email.clone()) {
            state.mailer.send_email(Email {
                to: user.email.clone(),
                subject: "Reservation cancelled".to_string(),
                body: format!(
                    "Hello, {}. Your reservation to room {} has been cancelled",
                    user.name, reservation.room
                ),
            });
        }
        db.reservations.remove(&id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn get_outbox(State(state): State<AppState>) -> Json<Vec<Email>> {
    let outbox = state.mailer.get_outbox();
    Json(outbox)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use serde_json::json;
    use tower::util::ServiceExt;

    async fn setup() -> Router {
        let state = AppState::new();
        Router::new()
            .route("/reservations", get(get_reservations))
            .route("/reservations", post(create_reservation))
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
