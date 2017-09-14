#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use rocket_contrib::{Json, Value};
use rocket::response::status::{Created, NoContent};
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

#[get("/movie/<id>", format = "application/json")]
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

#[post("/", format = "application/json", data = "<movie>")]
fn new(movie: Json<Movie>) -> Result<Created<Json<Movie>>, diesel::result::Error> {
    let conn = establish_connection();
    let x = movie::create_movie(&conn, movie.0)?;
    let url = format!("/movie/{}", x.id);
    Ok(Created(url, Some(Json(x))))
}

#[delete("/movie/<id>")]
fn movie_delete(id: i32) -> Result<NoContent, diesel::result::Error> {
    let conn = establish_connection();
    movie::delete_movie(&conn, id)?;
    Ok(NoContent)
}

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
        .mount("/", routes![get_all, get, new, movie_delete])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
