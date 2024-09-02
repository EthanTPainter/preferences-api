use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientQuery {
  pub limit: Option<u32>,
  pub offset: Option<u32>
}

#[derive(Deserialize)]
pub struct PreferenceTypeQuery {
  pub limit: Option<u32>,
  pub offset: Option<u32>
}

#[derive(Deserialize)]
pub struct PreferencesQuery {
  pub limit: Option<u32>,
  pub offset: Option<u32>
}