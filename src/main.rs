use axum::{extract::Path, http::StatusCode, routing::{get, patch, post}, Json, Router};

use axum_extra::extract::Query;
use request::{
  body::{
    ClientBody,
    ClientUpdateBody,
    PreferenceTypeBody,
    PreferencesBody},
  query::{
    ClientQuery,
    PreferenceTypeQuery
  }
};
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
pub mod db;

#[tokio::main]
async fn main() {
  // Router setup
  let app = Router::new()
    .route("/clients", get(get_clients))
    .route("/clients", post(create_client))
    .route("/clients/:client_id", get(get_client_by_id))
    .route("/clients/:client_id", patch(update_client))
    .route("/clients/:client_id/preference-types", get(get_preference_types))
    .route("/clients/:client_id/preference-types", post(create_preference_type))
    .route("/clients/:client_id/preference-types/:type_name", get(get_preference_type_by_name))
    .route("/clients/:client_id/preference-types/:type_name", patch(update_preference_type))
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
  Query(query): Query<ClientQuery>
) -> (StatusCode, Json<ClientPaginatedResponse>) {
  println!("Query parameters for GET /clients: limit ({}), offset ({})", query.limit, query.offset);
  (StatusCode::OK, Json(ClientPaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    clients: Vec::new()
  }))
}

async fn get_client_by_id(
  Query(query): Query<ClientQuery>,
  Path(client_id): Path<String>
) -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

async fn create_client(
  Json(payload): Json<ClientBody>
) -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

async fn update_client(
  Path(client_id): Path<String>,
  Json(payload): Json<ClientUpdateBody>
) -> (StatusCode, Json<ClientResponse>) {
  (StatusCode::OK, Json(ClientResponse { value: Vec::new()} ))
}

// Preference Types
async fn get_preference_types(
  Query(query): Query<PreferenceTypeQuery>
) -> (StatusCode, Json<PreferenceTypePaginatedResponse>) {
  (StatusCode::OK, Json(PreferenceTypePaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    preference_types: Vec::new()
  }))
}

async fn get_preference_type_by_name(
  Path((client_id, type_name)): Path<(String, String)>
) -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

async fn create_preference_type(
  Path(client_id): Path<String>,
  Json(payload): Json<PreferenceTypeBody>
) -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

async fn update_preference_type(
  Path((client_id, type_name)): Path<(String, String)>,
  Json(payload): Json<PreferenceTypeBody>
) -> (StatusCode, Json<PreferenceTypeResponse>) {
  (StatusCode::OK, Json(PreferenceTypeResponse { value: Vec::new()} ))
}

// Preferences
async fn get_preferences(
  Path((client_id, type_name)): Path<(String, String)>
) -> (StatusCode, Json<PreferencesPaginatedResponse>) {
  (StatusCode::OK, Json(PreferencesPaginatedResponse {
    count: 0,
    limit: 10,
    offset: 0,
    preferences: Vec::new()
  }))
}

async fn create_preferences(
  Path((client_id, type_name)): Path<(String, String)>,
  Json(payload): Json<PreferencesBody>
) -> (StatusCode, Json<PreferencesResponse>) {
  (StatusCode::OK, Json(PreferencesResponse { value: Vec::new()} ))
}

async fn update_preferences(
  Path((client_id, type_name)): Path<(String, String)>,
  Json(payload): Json<PreferencesBody>
) -> (StatusCode, Json<PreferencesResponse>) {
  (StatusCode::OK, Json(PreferencesResponse { value: Vec::new()} )) 
}