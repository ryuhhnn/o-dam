pub trait Revision {
  fn id(&self) -> &str;
  fn version(&self) -> &i32;
  fn current(&self) -> &bool;
  fn file_name(&self) -> &str;
  fn as_revision(&self) -> &dyn Revision;
}

pub struct RevisionData {
  id: String,
  version: i32,
  // data: File, // TODO: fix this to be the correct type
  current: bool,
  file_name: String,
}

impl Revision for RevisionData {
  fn id(&self) -> &str {
    &self.id
  }
  fn version(&self) -> &i32 {
    &self.version
  }
  fn current(&self) -> &bool {
    &self.current
  }
  fn file_name(&self) -> &str {
    &self.file_name
  }
  fn as_revision(&self) -> &dyn Revision {
    self
  }
}

impl RevisionData {
  pub fn new(id: &str, version: &i32, current: &bool, file_name: &str) -> RevisionData {
    RevisionData {
      id: id.to_owned(),
      version: version.to_owned(),
      current: current.to_owned(),
      file_name: file_name.to_owned(),
    }
  }
}
