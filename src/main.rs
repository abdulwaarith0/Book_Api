#![allow(dead_code)]
extern crate diesel;
extern crate diesel_codegen;
use diesel::pg::PgConnection;
use diesel::prelude::*;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("SET DATABASE_URL");
    let conn = &mut PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, conn) {
        println!("success");
    } else {
        println!("failed");
    }
}
