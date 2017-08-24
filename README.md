# Movie API w/ Rust & Rocket

## Setup

1. Ensure you're currently running Rust Nightly. [Here are the installation instructions](https://doc.rust-lang.org/1.13.0/book/nightly-rust.html).
2. Clone/Fork the repo.
3. Set up `.env` file with local postgres credentials. ex. `DATABASE_URL=postgres://user:password@localhost:XXXX/movie-api`. (Don't forget to add this to `.gitignore`)
4. Install the [Diesel](http://diesel.rs/) CLI with `cargo install diesel_cli`.
5. Run `diesel setup` which will setup the database if it's hasn't already been.
6. Run `cargo run` to start the application.
