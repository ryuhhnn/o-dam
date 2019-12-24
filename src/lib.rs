#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket_contrib;

pub mod graphql;
mod model;
pub mod neo;
pub mod routes;
