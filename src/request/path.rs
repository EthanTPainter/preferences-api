use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientPathParams {
  pub limit: u32,
  pub offset: u32
}

#[derive(Deserialize)]
pub struct PreferencePathParams {
  pub limit: u32,
  pub offset: u32
}

#[derive(Deserialize)]
pub struct PreferencesPathParams {
  pub limit: u32,
  pub offset: u32
}