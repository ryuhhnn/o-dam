use crate::model::asset::Asset;
use crate::model::collection::Collection;
use crate::model::revision::Revision;
use crate::model::store::Store;

impl juniper::Context for Store {}

graphql_object!(<'a> &'a dyn Collection: Store as "Collection" |&self| {
  description: "A collection"

  interfaces: [&dyn Collection]

  field id() -> &str as "The id of the collection" {
    self.id()
  }

  field uuid() -> &str as "The uuid of the collection" {
    self.uuid()
  }

  field name() -> &str as "The uuid of the collection" {
    self.name()
  }

  field description() -> &Option<String> as "The description of the collection" {
    self.description()
  }
});

graphql_object!(<'a> &'a dyn Asset: Store as "Asset" |&self| {
  description: "An asset"

  interfaces: [&dyn Asset]

  field id() -> &str as "The id of the asset" {
    self.id()
  }

  field uuid() -> &str as "The uuid of the asset" {
    self.uuid()
  }

  field file_name() -> &str as "The file name of the asset" {
    self.file_name()
  }

  field extension() -> &str as "The extension of the asset" {
    self.extension()
  }

  field full_path() -> &str as "The full path of the asset" {
    self.full_path()
  }

  field description() -> &Option<String> as "The description of the asset" {
    self.description()
  }
});

graphql_object!(<'a> &'a dyn Revision: Store as "Revision" |&self| {
  description: "A revision"

  interfaces: [&dyn Revision]

  field id() -> &str as "The id of the revision" {
    self.id()
  }

  field version() -> &i32 as "The version of the revision" {
    self.version()
  }

  field current() -> &bool as "Whether or not revision is the current version" {
    self.current()
  }

  field file_name() -> &str as "The file name of the revision" {
    self.file_name()
  }
});

graphql_object!(Store: Store as "Query" |&self| {
  description: "The root query object of the schema"

  field collection(id: String as "id of the collection") -> Option<&dyn Collection> {
      self.get_collection(&id)
  }
});
