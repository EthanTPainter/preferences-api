use serde::Serialize;

#[derive(Serialize)]
pub struct ClientPaginatedResponse {
  pub limit: u32,
  pub offset: u32,
  pub count: u32,
  pub clients: Vec<u32>
}

#[derive(Serialize)]
pub struct ClientResponse {
  pub value: Vec<u32>
}

#[derive(Serialize)]
pub struct PreferenceTypePaginatedResponse {
  pub limit: u32,
  pub offset: u32,
  pub count: u32,
  pub preference_types: Vec<u32>
}

#[derive(Serialize)]
pub struct PreferenceTypeResponse {
  pub value: Vec<u32>
}

#[derive(Serialize)]
pub struct PreferencesPaginatedResponse {
  pub limit: u32,
  pub offset: u32,
  pub count: u32,
  pub preferences: Vec<u32>
}

#[derive(Serialize)]
pub struct PreferencesResponse {
  pub value: Vec<u32>
}