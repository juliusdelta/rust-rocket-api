# Movie API w/ Rust & Rocket

## Requirements

- Rust Nightly
- Postgres installed

## Setup

1. Ensure you're currently running Rust Nightly. [Here are the installation instructions](https://doc.rust-lang.org/1.13.0/book/nightly-rust.html).
2. Clone/Fork the repo.
3. Set up `.env` file with local postgres credentials. ex. `DATABASE_URL=postgres://user:password@localhost:XXXX/movie-api`. (Don't forget to add this to `.gitignore`)
4. Install the [Diesel](http://diesel.rs/) CLI with `cargo install diesel_cli`.
5. Run `diesel setup` which will setup the database if it's hasn't already been.
6. Run `cargo run` to start the application.

## Seeding the Database

As I haven't written a proper seeding script yet you can seed the database manually for the time being. Here's a quick run through of doing so. 

**Disclaimer:** You have to have finished at least step 5 in the above setup steps for the table to be created.

In terminal:

```
$ psql movie-api

$ INSERT INTO movies (id, title, director, rating)
VALUES (1, 'The Matrix', 'The Wachowskis', 'R'),
(2, '2001: A Space Odyssey', 'Stanley Kubrick', 'PG-13');

$ \q #+to quit psql
```

That should give you some temp seed data until I can get a seeding script run.
