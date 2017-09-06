# Movie API w/ Rust & Rocket

## Requirements

- Rust Nightly
- PostgreSQL installed

## Setup & Test

1. Ensure you're currently running Rust Nightly. [Here are the installation instructions](https://doc.rust-lang.org/1.13.0/book/nightly-rust.html).
2. Clone the repo down.
3. Set up `.env` file with local postgres credentials. See `.example.env` for a quick example. (It's already in the .gitignore for this repo)
   - I have a small guide on finding your local Postgres info on [my blog](http://localhost:4000/2017/08/31/find-local-postgres-info).
   - If you've just installed postgres locally, chances are you don't have a password. See `.example.env` for reference.
4. Install the [Diesel](http://diesel.rs/) CLI with `cargo install diesel_cli`.
5. Run `diesel setup` which will setup the database if it's hasn't already been.
6. In `./migrations/20170810032025_create_movie/up.sql` there is a seed database function you can uncomment to seed the database if you want.
7. Run `diesel migration run` to create the table and seed database (if you uncommented out the `INSERT` statement).
   - If you seed the database, you can verify the objects in there by running `psql movie-api` then `SELECT * FROM movies;` and then `\q` to quit.
7. Run `cargo run` to start the application.
8. Since it's just an API expecting GET and POST requests. So you can can run:
   - `curl -X GET http://localhost:8000/` to make a GET request to the local server for all movies in the database or `http://localhost:8000/movie/1` where `1` is the `id` of the movie in the database.
   - Alternatively, you can navigate to `http://localhost:8000` in the browser, adjust your request headers to send for `application/json` and then see the response back if you'd rather do that.

## Code Direction

This is just a simple project I anticipate making a robust guide for. There is a significant amount of things to be done including finishing out the `POST` and `PUT` requests, adding database relationships for things like directors and actors for movies, and then ultimately a frontend of some kind. Currently I haven't written out a proper roadmap, however, if there's something you'd like to add, suggest, or fix, feel free to open an issue or a PR and I will respond. 

I do plan to document this thoroughly as this is my first Rust project, and a lot of learning resources assume prior knowledge of C++ or C to be productive quickly. I'm sure there are a good amount of exceptions to that, but that's been my personal experience thus far. 
