# ThingRanker API
API for the ThingRanker web application.

## Cargo Setup
To build the API, ensure to install Cargo from [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Local DB Setup
* Install PostgreSQL 16 or higher from [here](https://www.postgresql.org/download/).
* Create an empty DB named 'thing_ranker' (or whatever you like).
* Configure user that will connect to 'thing_ranker' if necessary. For instance, username / password.

## Running the API Locally
Create a file named `config.yml` in the root of the project with the following fields:
```yml
port: <port>                          # Port the API runs on. When running locally, this should be '8080'.
asset_store_type: <asset_store_type>  # Where to store uploaded assets. Can be 'local' or 's3'.
cors:                                 # All CORS related settings
  allowed_origin: <allowed_origin>    # Domain of UI application. When running locally, this should be 'http://localhost:3000'.
db:                                   # DB connection data
  name: thing_ranker                  # DB name. This is often just 'thing_ranker'.
  user: <user>                        # DB user. This is often just 'postgres'.
  password: <password>                # DB user's password. This is often just 'postgres'.
  host: <host>                        # DB host. When running locally, this should be 'localhost'.
  port: <port>                        # DB port. Default value for postgres is 5432.
auth:                                 # Auth token settings
  jwt_secret: <jwt_secret>            # Value used for signing auth tokens. Should NOT be in source control.
  jwt_exp_secs: <jwt_exp_secs>        # Time it takes for an auth token to expire in seconds. 86400 for 24 hours.
oidc:                                 # OIDC (Open ID Connect) settings
  google:                             # Google OIDC settings
    client_id: <client_id>            # Google's client ID of the app. Same value as the one in the UI project.
roles:                                # List of roles for accounts, granted at app startup.
  - email: root@thingranker.com
    role: root
  - email: admin@thingranker.com
    role: admin 
  - email: basic@thingranker.com
    role: basic
```

## Generating Test Auth Tokens
Run the following command to generate a list of auth tokens for all accounts in the `roles` section of the config file:
```bash
cargo run --bin gen_auth_tokens
```

Run the following command:
```bash
cargo run  # Runs API in dev mode. No optimizations.
```
Or, run the following command:
```bash
cargo run --release # Runs API in release mode.
```

## DB Migrations
SQL migrations are handled by the [SQLX](https://docs.rs/sqlx/latest/sqlx/) library.
Migration scripts live under /api/migrations, and are run when the API starts up.

## Resetting DB Schema
If you'd like to blow away all data in the event you need to modify a migration script you're testing,
you can run the following command on Linux:
```bash
sudo -u postgres psql -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public" thing_ranker
```
