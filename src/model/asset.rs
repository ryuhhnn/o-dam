pub trait Asset {
  fn id(&self) -> &str;
  fn uuid(&self) -> &str;
  fn file_name(&self) -> &str;
  fn extension(&self) -> &str;
  fn full_path(&self) -> &str;
  fn description(&self) -> &Option<String>;
  fn as_asset(&self) -> &dyn Asset;
}

pub struct AssetData {
  id: String,
  uuid: String,
  file_name: String,
  extension: String,
  full_path: String,
  description: Option<String>,
}

impl Asset for AssetData {
  fn id(&self) -> &str {
    &self.id
  }
  fn uuid(&self) -> &str {
    &self.uuid
  }
  fn file_name(&self) -> &str {
    &self.file_name
  }
  fn extension(&self) -> &str {
    &self.extension
  }
  fn full_path(&self) -> &str {
    &self.full_path
  }
  fn description(&self) -> &Option<String> {
    &self.description
  }
  fn as_asset(&self) -> &dyn Asset {
    self
  }
}

impl AssetData {
  pub fn new(
    id: &str,
    uuid: &str,
    file_name: &str,
    extension: &str,
    full_path: &str,
    description: Option<&str>,
  ) -> AssetData {
    AssetData {
      id: id.to_owned(),
      uuid: uuid.to_owned(),
      file_name: file_name.to_owned(),
      extension: extension.to_owned(),
      full_path: full_path.to_owned(),
      description: description.map(|d| d.to_owned()),
    }
  }
}
