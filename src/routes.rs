use crate::graphql::schema::{Context, MutationRoot, QueryRoot};
use crate::neo::NeoStore;
use juniper::RootNode;
use rocket::{get, post, response::content, State};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
  juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
  context: NeoStore,
  request: juniper_rocket::GraphQLRequest,
  schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(
    &schema,
    &Context {
      connection: context,
    },
  )
}
