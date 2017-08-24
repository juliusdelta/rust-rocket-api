extern crate serde;
extern crate serde_json;

use diesel::result::Error;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;
use schema::movies;

pub fn get_movie(conn: &PgConnection, id: i32) -> Result<Movie, Error> {
    movies::table
        .find(id)
        .first::<Movie>(conn)
}

pub fn get_movies(conn: PgConnection) -> Result<Vec<Movie>, Error> {
    movies::table
        .load::<Movie>(&conn)
}

// pub fn create_movie(conn: &PgConnection, movie: <Movie>) -> Result<Movie, Error> {
//     diesel::insert(&movie)
//         .into(movies::table)
//         .get_result(conn)
// }

// pub fn delete_movie(conn: &PgConnection, id: i32) -> Result<usize, Error> {
//     diesel::delete(movies::table.find(id))
//         .execute(conn)
// }

// pub fn update_movie(conn: &PgConnection, id: i32, updated_move: Json<Movie>) -> Result<Movie, Error> {
//     diesel::update(movies::table.find(id))
//         .set(&updated_movie)
//         .get_result::<Movie>(conn)
// }
