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
use schema::movies;

pub mod schema;
pub mod models;

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
    let movie = movies::table
        .find(id)
        .first::<Movie>(&conn)?;
    Ok(Json(movie))
}

#[get("/", format = "application/json")]
fn get_all() -> Result<Json<Vec<Movie>>, diesel::result::Error> {
    let conn: PgConnection = establish_connection();
    let movies = movies::table
        .load::<Movie>(&conn)?;
    Ok(Json(movies))
}

#[post("/", format = "application/json", data = "<movie>")]
fn new(movie: Json<NewMovie>) -> Result<Created<Json<Movie>>, diesel::result::Error> {
    let conn = establish_connection();
    let new_movie = NewMovie {
        title: movie.0.title,
        director: movie.0.director,
        rating: movie.0.rating
    };

    let movie: Movie = diesel::insert(&new_movie)
        .into(movies::table)
        .get_result(&conn)?;

    let url = format!("/movie/{}", movie.id);
    Ok(Created(url, Some(Json(movie))))
}

#[delete("/movie/<id>")]
fn movie_delete(id: i32) -> Result<NoContent, diesel::result::Error> {
    let conn = establish_connection();
    diesel::delete(movies::table.find(id))
        .execute(&conn)?;
    Ok(NoContent)
}

#[patch("/movie/<id>", format = "application/json", data = "<movie>")]
fn movie_edit(id: i32, movie: Json<Movie>) -> Result<Json<Movie>, diesel::result::Error> {
    let conn = establish_connection();
    let movie = diesel::update(movies::table.find(id))
        .set(&movie.0)
        .get_result::<Movie>(&conn)?;
    Ok(Json(movie))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource not found"
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get_all, get, new, movie_delete, movie_edit])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
