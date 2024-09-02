use axum::{http::StatusCode, routing::{get, patch, post}, Json, Router};

use response::body::{
  ClientPaginatedResponse,
  ClientResponse,
  PreferenceTypePaginatedResponse,
  PreferenceTypeResponse,
  PreferencesPaginatedResponse,
  PreferencesResponse
};

pub mod request;
pub mod response;

#[tokio::main]
async fn main() {
  // Router setup
  let app = Router::new()
    .route("/clients", get(get_clients))
    .route("/clients/:client_id", get(get_client_by_id))
    .route("/clients", post(create_client))
    .route("/clients/:client_id", patch(update_client)) 
    .route("/clients/:client_id/preference-types", get(get_preference_types))
    .route("/clients/:client_id/preference-types/:type", get(get_preference_type_by_name))
    .route("/clients/:client_id/preference-types", post(create_preference_type))
    .route("/clients/:client_id/preference-types/:type", patch(update_preference_type))
    .route("/clients/:client_id/preference-types/:type/preferences", get(get_preferences))
    .route("/clients/:client_id/preference-types/:type/preferences", post(create_preferences))
    .route("/clients/:client_id/preference-types/:type/preferences", patch(update_preferences));

  // Start app on local
  println!("Running on http://localhost:3000");
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

// Clients
async fn get_clients(
  // Query(query): Query<ClientQuery>
) -> (StatusCode, Json<ClientPaginatedResponse>) {
  (StatusCode::OK, Json(ClientPaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    clients: Vec::new()
  }))
}

async fn get_client_by_id() -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

async fn create_client() -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

async fn update_client() -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

// Preference Types
async fn get_preference_types() -> (StatusCode, Json<PreferenceTypePaginatedResponse>) {
  (StatusCode::OK, Json(PreferenceTypePaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    preference_types: Vec::new()
  }))
}

async fn get_preference_type_by_name() -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

async fn create_preference_type() -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

async fn update_preference_type() -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

// Preferences
async fn get_preferences() -> (StatusCode, Json<PreferencesPaginatedResponse>) {
  (StatusCode::OK, Json(PreferencesPaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    preferences: Vec::new()
  }))
}

async fn create_preferences() -> (StatusCode, Json<PreferencesResponse>) {
  (StatusCode::OK, Json(PreferencesResponse { value: Vec::new()} ))
}

async fn update_preferences() -> (StatusCode, Json<PreferencesResponse>) {
  (StatusCode::OK, Json(PreferencesResponse { value: Vec::new()} )) 
}