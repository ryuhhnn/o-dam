use crate::neo::NeoStore;

pub struct Context {
  pub connection: NeoStore,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context as "Query" |&self| {
  description: "The root query object of the schema"

  // field collection(id: String as "id of the collection") -> Option<&dyn Collection> {
  //     self.get_collection(&id)
  // }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context | &self | {
  description: "The root mutation object of the schema"

  // field collection(id: String as "id of the collection") -> Option<&dyn Collection> {
  //     self.get_collection(&id)
  // }
});
