## Simple Setup Guide

## Develop With Docker

1. Install latest version of Docker on your machine
2. Create a file called `.env` in the root of this project and copy over the contents of `.env.example` to it
3. After that run `docker compose up --scale nginx=0` (or if you have access to the `make` command, run `make dc-start-local` instead) in a terminal and monitor the logs to see when the Rust has finished compiling
4. Once you see a message in the logs that reads something like `Running target/debug/rust-auth`, you should be good to go
   - I recommend running `docker logs --tail 10 -f rust-auth` to see the last few lines of the logs if you chose to run `docker compose` with the `-d` flag

## Endpoints

Upon running the server, you should have access to a few endpoints:

### `POST /register`:

Request:

```json
{
  "username": "String",
  "password": "String",
  "email": "String"
}
```

Response:

```json
{
  "username": "String",
  "email": "String",
  "token": "String",
  "id": "Number"
}
```

### `POST /login`:

Request:

```json
{
  "username": "String",
  "password": "String",
}
```

Response:

```json
{
  "username": "String",
  "email": "String",
  "token": "String",
  "id": "Number"
}
```

### `GET /profile`:

Request:

Just `Bearer` token in the header

Response:

```json
{
  "username": "String",
  "email": "String",
  "id": "Number"
}
```

### `POST /logout`:

Request:

Just `Bearer` token in the header

Response:

```json
{ }
```

## Migrations

The steps to creating a migration and create an entity from it are as follows:

1. First, make sure the `DATABASE_URL` variable is set in your root `.env` file
2. Then, create the new migration file by running `sea-orm-cli migrate generate "name of migration"`
   - this will create a shell migration file and update the pointer to the
     latest migration. now, go ahead and edit the migration file
3. Run the migrations against the database with `sea-orm-cli migrate up`
4. After the migrations have been applied, generate the entities with `sea-orm-cli generate entity -o entity/src`
5. Now, you're ready to start writing Rust code to interact with the database
