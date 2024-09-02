use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientQuery {
  pub limit: u32,
  pub offset: u32
}

#[derive(Deserialize)]
pub struct PreferenceTypeQuery {
  pub limit: u32,
  pub offset: u32
}

#[derive(Deserialize)]
pub struct PreferencesQuery {
  pub limit: u32,
  pub offset: u32
}