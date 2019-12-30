use confy;
use juniper::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
  pub fs_adapter: String,
}

impl Default for ConfigData {
  fn default() -> Self {
    ConfigData {
      fs_adapter: "local".to_string(),
    }
  }
}

impl ConfigData {
  pub fn read() -> Self {
    let config: ConfigData = confy::load("odam_config").unwrap();
    config
  }
}

#[derive(Debug)]
pub enum ConfigError {
  FileReadError,
}

impl<'a, 'r> FromRequest<'a, 'r> for ConfigData {
  type Error = ConfigError;

  fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let config = confy::load("odam_config");
    match config {
      Ok(config_data) => Outcome::Success(config_data),
      Err(_e) => Outcome::Failure((Status::BadRequest, ConfigError::FileReadError)),
    }
  }
}
