use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientBody {
  pub client_id: String,
  pub client_name: String
}


#[derive(Deserialize)]
pub struct ClientUpdateBody {
  pub client_name: String
}

#[derive(Deserialize)]
pub struct PreferenceTypeBody {
  pub limit: u32
}

#[derive(Deserialize)]
pub struct PreferencesBody {
  pub limit: u32
}