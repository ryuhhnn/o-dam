pub trait Collection {
  fn id(&self) -> &str;
  fn uuid(&self) -> &str;
  fn name(&self) -> &str;
  fn description(&self) -> &Option<String>;
  fn as_collection(&self) -> &dyn Collection;
}

pub struct CollectionData {
  id: String,
  uuid: String,
  name: String,
  description: Option<String>,
}

impl Collection for CollectionData {
  fn id(&self) -> &str {
    &self.id
  }
  fn uuid(&self) -> &str {
    &self.uuid
  }
  fn name(&self) -> &str {
    &self.name
  }
  fn description(&self) -> &Option<String> {
    &self.description
  }
  fn as_collection(&self) -> &dyn Collection {
    self
  }
}

impl CollectionData {
  pub fn new(id: &str, uuid: &str, name: &str, description: Option<&str>) -> CollectionData {
    CollectionData {
      id: id.to_owned(),
      uuid: uuid.to_owned(),
      name: name.to_owned(),
      description: description.map(|d| d.to_owned()),
    }
  }
}
