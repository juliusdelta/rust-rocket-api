#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate dotenv;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use rocket_contrib::{Json, Value};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::*;

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
fn get(id: i32) -> Result<Json<Movie>, diesel::result::Error> {
    let conn: PgConnection = establish_connection();
    let movie = movie::get_movie(&conn, id)?;
    Ok(Json(movie))
}

#[get("/", format = "application/json")]
fn get_all() -> Result<Json<Vec<Movie>>, diesel::result::Error> {
    let conn: PgConnection = establish_connection();
    let movies = movie::get_movies(conn)?;
    Ok(Json(movies))
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
        .mount("/", routes![get_all, get])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
