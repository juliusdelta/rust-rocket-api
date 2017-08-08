#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

type ID = usize;

type MovieMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Movie {
    id: Option<ID>,
    title: String
}

#[post("/<id>", format = "application/json", data = "<movie>")]
fn new(id: ID, movie: Json<Movie>, map: State<MovieMap>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        Json(json!({
            "status": "error",
            "reason": "ID exists. Try put."
        }))
    } else {
        hashmap.insert(id, movie.0.title);
        Json(json!({"status": "ok"}))
    }
}

#[put("/<id>", format = "application/json", data = "<movie>")]
fn update(id: ID, movie: Json<Movie>, map: State<MovieMap>) -> Option<Json<Value>> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, movie.0.title);
        Some(Json(json!({"status": "ok"})))
    } else {
        None
    }
}

#[get("/<id>", format = "application/json")]
fn get(id: ID, map: State<MovieMap>) -> Option<Json<Movie>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|titles| {
        Json(Movie {
            id: Some(id),
            title: titles.clone()
        })
    })
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
        .mount("/movie", routes![new, update, get])
        .catch(errors![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
