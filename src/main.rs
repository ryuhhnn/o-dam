#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket_contrib;

mod model;
mod neo;
mod schema;
use juniper::RootNode;
use rocket::{response::content, State};

pub type Schema = RootNode<'static, schema::schema::QueryRoot, schema::schema::MutationRoot>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: neo::NeoStore,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(
        &schema,
        &schema::schema::Context {
            connection: context,
        },
    )
}

fn main() {
    rocket::ignite()
        .attach(neo::NeoStore::fairing())
        .manage(Schema::new(
            schema::schema::QueryRoot,
            schema::schema::MutationRoot,
        ))
        .mount("/", rocket::routes![graphiql, post_graphql_handler])
        .launch();
}
