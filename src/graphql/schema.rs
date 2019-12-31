use crate::config::ConfigData;
use crate::model::collection::Collection;
use crate::neo::NeoStore;
use crate::storage::persist::Persist;
use juniper::FieldResult;
use rocket_contrib::databases::rusted_cypher::Statement;
use uuid::Uuid;

pub struct Context {
  pub connection: NeoStore,
  pub config: ConfigData,
}

impl juniper::Context for Context {}

graphql_object!(Collection: () |&self| {
  description: "Collection"

  field uuid() -> &str as "The unique ID of the collection" {
    &self.uuid
  }

  field name() -> &str as "Name of the collection" {
    &self.name
  }

  field description() -> Option<String> as "Description of the collection" {
    self.description.to_owned()
  }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context as "Query" |&self| {
  description: "The root query object of the schema"

  field collections(&executor) -> FieldResult<Vec<Collection>> {
    let mut results = Vec::new();
    let statement = Statement::new("MATCH (c:Collection) RETURN c.uuid, c.name, c.description");
    let query = executor.context().connection.exec(statement)?;

    for result in query.rows() {
      let uuid = result.get("c.uuid")?;
      let name = result.get("c.name")?;
      let description = result.get("c.description")?;
      results.push(Collection { uuid: uuid, name: name, description: description})
    }

    Ok(results)
  }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context | &self | {
  description: "The root mutation object of the schema"

  field collection(
    &executor,
    name: String as "id of the collection",
    description: Option<String> as "description of the collection"
  ) -> FieldResult<Collection> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let fs_adapter = executor.context().config.fs_adapter.as_str();
    Persist::create_folder(fs_adapter, uuid.as_str())?;

    let statement = Statement::new(
        "CREATE (c:Collection {uuid: {uuid}, name: {name}, description: {description}}) RETURN c.uuid, c.name, c.description"
      )
      .with_param("uuid", &uuid).unwrap()
      .with_param("name", &name).unwrap()
      .with_param("description", &description).unwrap();
    let query = executor.context().connection.exec(statement)?;
    let result = query.rows().nth(0).unwrap();

    let uuid: String = result.get("c.uuid")?;
    let name: String = result.get("c.name")?;
    let description = match result.get("c.description") {
      Ok(description) => Some(description),
      Err(_e) => None,
    };

    Ok(Collection { uuid: uuid, name: name, description: description})
  }
});
