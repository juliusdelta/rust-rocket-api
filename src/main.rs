#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use rocket_contrib::{Json, Value};
use rocket::State;
use diesel::prelude::*;
#[macro_use]
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::*;
use movie::{get_movie};
use rocket::error::Error;

pub mod schema;
pub mod models;
pub mod movie;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}.", database_url))
}

#[get("/<id>", format = "application/json")]
fn get<'a>(id: i32) -> Result<Json<Movie>, &'static str> {
    let conn: PgConnection = establish_connection();
    let movie: Result<Movie, diesel::result::Error> = movie::get_movie(&conn, id);
    match movie {
        Ok(value) => Json(value),
        Err(e) => println!("{}", e),
    }
}

// #[post("/<id>", format = "application/json", data = "<movie>")]

// #[put("/<id>", format = "application/json", data = "<movie>")]

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource not found"
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/movie", routes![get])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
