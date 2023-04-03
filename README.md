# Tuiter Backend in Rust

This is a backend for Tuiter, a twitter inspired fake social network for education purposes.
It is written in Rust and uses the [axum](https://crates.io/crates/axum) web application framework, [rusqulte](https://crates.io/crates/rusqlite) for the database and [jsonwebtoken](https://crates.io/crates/jsonwebtoken) for authentication.

## API Documentation

- /tweets
  - GET: list last 50 tuites
  - POST: create a new tuite
- /tweets/:id
  - GET: find a tuite by its id
  - DELETE: delete a tuite by its id
- /tweet/:id/like
  - POST: add +1 like to a tuite
  - DELETE: add -1 like to a tuite

## Running

To run the backend, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

Then, you can run the backend with:

```bash
cargo run --release
```

## Testing

You can run the tests with:

```bash
cargo test
```
