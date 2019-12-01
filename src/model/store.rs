use std::collections::HashMap;

use crate::model::asset::{Asset, AssetData};
use crate::model::collection::{Collection, CollectionData};
use crate::model::revision::{Revision, RevisionData};

pub struct Store {
  collections: HashMap<String, CollectionData>,
  assets: HashMap<String, AssetData>,
  revisions: HashMap<String, RevisionData>,
}

impl Store {
  pub fn new() -> Store {
    let mut collections = HashMap::new();
    let mut assets = HashMap::new();
    let mut revisions = HashMap::new();

    collections.insert(
      "1".to_owned(), // ID
      CollectionData::new(
        "1",    // ID
        "uuid", // UUID
        "Test Name",
        None,
      ),
    );

    collections.insert(
      "2".to_owned(), // ID
      CollectionData::new(
        "2",    // ID
        "uuid", // UUID
        "Test Name",
        Some("Test Description"),
      ),
    );

    assets.insert(
      "1".to_owned(), // ID
      AssetData::new(
        "1",    // ID
        "uuid", // UUID
        "file-name",
        ".jpg",
        "/full/path",
        None,
      ),
    );

    revisions.insert(
      "1".to_owned(), // ID
      RevisionData::new(
        "1", // ID
        &1,
        &true,
        "file-name",
      ),
    );

    Store {
      collections: collections,
      assets: assets,
      revisions: revisions,
    }
  }

  pub fn get_collection(&self, id: &str) -> Option<&dyn Collection> {
    self.collections.get(id).map(|c| c as &dyn Collection)
  }

  pub fn get_asset(&self, id: &str) -> Option<&dyn Asset> {
    self.assets.get(id).map(|a| a as &dyn Asset)
  }

  pub fn get_revision(&self, id: &str) -> Option<&dyn Revision> {
    self.revisions.get(id).map(|r| r as &dyn Revision)
  }
}
