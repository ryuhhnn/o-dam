#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;
use juniper::{EmptyMutation, RootNode};
use rocket::{response::content, State};

type Schema = RootNode<'static, model::store::Store, EmptyMutation<model::store::Store>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

// TODO: not sure why we need this, come back to it
#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<model::store::Store>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<model::store::Store>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(model::store::Store::new())
        .manage(Schema::new(
            model::store::Store::new(),
            EmptyMutation::<model::store::Store>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
