use super::fs_adapter::create_folder_fs;
// use std::io::{Error, ErrorKind};

pub struct Persist;

impl Persist {
  pub fn create_folder(fs_adapter: &str, folder: &str) -> Result<(), &'static str> {
    match fs_adapter {
      "local" => create_folder_fs(folder).unwrap(),
      _ => return Err("Unable to read fs_adapter from config."),
    };

    Ok(())
  }
}
