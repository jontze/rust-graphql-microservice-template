# Rust GraphQL Webserver Template

This template repo is used for the fast setup of a actix-web server with GraphQL, SQLx and Postgres under the hood.

The following technologies are used:
* Rust
* Actix-Web
* GraphQL
* SQLX
* Postgres

For a fast setup:
1. Clone the repo
2. Run `cargo install sqlx-cli`
3. Run `docker-compose up -d`
4. Create your `.env` file based on the [example](./.env.example)
5. Optional edit or add migrations and apply them with `sqlx migrate run`
6. Starte the server with `cargo run`
7. Go to `http://localhost:8080/graphiql`
