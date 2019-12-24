#![feature(decl_macro, proc_macro_hygiene)]

use odam::graphql::schema::{MutationRoot, QueryRoot};
use odam::neo::NeoStore;
use odam::routes::{self, Schema};
use rocket::routes;

fn main() {
  rocket::ignite()
    .attach(NeoStore::fairing())
    .manage(Schema::new(QueryRoot, MutationRoot))
    .mount("/", routes![routes::graphiql, routes::post_graphql_handler])
    .launch();
}
