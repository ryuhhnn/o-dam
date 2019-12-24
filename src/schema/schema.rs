use crate::model::collection::Collection;
use crate::neo::NeoStore;
use juniper::FieldResult;
use rocket_contrib::databases::rusted_cypher::Statement;
use uuid::Uuid;

pub struct Context {
  pub connection: NeoStore,
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
    let uuid = Uuid::new_v4();
    let statement = Statement::new(
        "CREATE (c:Collection {uuid: {uuid}, name: {name}, description: {description}})"
      );
      // .with_param("uuid", uuid).unwrap()
      // .with_param("name", name).unwrap()
      // .with_param("description", description).unwrap();
    let result = executor.context().connection.exec(statement)?;

    result.rows();

    Ok(Collection { uuid: String::from("asdf"), name: String::from("asdf"), description: Some(String::from("asdf"))})
  }
});
