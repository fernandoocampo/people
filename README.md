# people

This is a microservice project built on Rust. The idea is to learn about building this type of applications with the language.

## Technologies

### Tokio
[Tokio](https://tokio.rs) is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications.

### Warp
[warp](https://docs.rs/warp/latest/warp/) is a super-easy, composable, web server framework for warp speeds.

## How to test it?

I added a Makefile, but feel free to run them as they are using cargo. I tried to add unit tests as much as possible to learn how to use them, I found many issues trying to test Warp due to lack of documentation, but I managed to do it.

Rust conventions suggest adding the unit tests in the same file where you have your code logic, but we're used to adding different files for that purpose, so I followed the Go convention.

```sh
make test
```

or

```sh
cargo test
```

## How to run it?

```sh
make run
```

or

```sh
RUST_LOG=debug cargo run
```

you will see something like this

```sh
➜  make run
RUST_LOG=debug LOG_SYSTEM=log4rs cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/people`
⏱️	Starting people api application...
🪵	Initializing logger...
LOG_SYSTEM: log4rs
2023-09-30T19:58:16.219960+02:00 INFO people::application::app - 🪵	Using log4rs
2023-09-30T19:58:16.220487+02:00 INFO people::application::app - 🗿	Starting database connection...
2023-09-30T19:58:16.320768+02:00 INFO people::application::app - 🔎	Initializing censorious mechanism...
2023-09-30T19:58:16.321201+02:00 INFO people::application::app - 🔮	Initializing people handler...
2023-09-30T19:58:16.321223+02:00 INFO people::application::app - 🪜 	Establishing API routes...
2023-09-30T19:58:16.321315+02:00 INFO people::application::app - 👥	Creating people endpoint: GET /people
2023-09-30T19:58:16.321379+02:00 INFO people::application::app - 👤	Creating get person endpoint: GET /people/{id}
2023-09-30T19:58:16.321392+02:00 INFO people::application::app - 👤	Creating update person endpoint: PUT /people
2023-09-30T19:58:16.321406+02:00 INFO people::application::app - 👤	Creating add person endpoint: POST /people
2023-09-30T19:58:16.321416+02:00 INFO people::application::app - 👤	Creating delete person endpoint: DELETE /people/{id}
2023-09-30T19:58:16.321532+02:00 INFO people::application::app - 🍏	Starting server at :3030
2023-09-30T19:58:16.321824+02:00 INFO warp::server - Server::run; addr=127.0.0.1:3030
2023-09-30T19:58:16.321910+02:00 INFO warp::server - listening on http://127.0.0.1:3030
```

once you finished just hit `ctrl + c`

* another possible values for RUST_LOG

error
warn
info
debug
trace

## How to call the API?

* Create a person endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"name":"Esme"}' \
-X POST http://localhost:3030/people

{"id":"f1601fc5-f0c9-4950-8017-e094b284cad9"}
```

* Calling get people endpoint

```sh
curl -X GET http://localhost:3030/people
[{"id":"1","name":"Luis"},{"id":"2","name":"Fernando"}]
```

with params

limit: The index of the last item which has to be returned
offset: The index of the first item which has to be returned

```sh
curl -X GET 'http://localhost:3030/people?limit=10&offset=0'

[{"id":"32fed5e3-4a2b-4dfb-82b1-56e5ebcc0ed9","name":"Esme"},{"id":"d3bc8246-53da-4275-b833-5feb4489741d","name":"Jorge"},{"id":"f1601fc5-f0c9-4950-8017-e094b284cad9","name":"Luis"}]
```

* Calling get a person endpoint

```sh
curl -X GET http://localhost:3030/people/32fed5e3-4a2b-4dfb-82b1-56e5ebcc0ed9

{"id":"32fed5e3-4a2b-4dfb-82b1-56e5ebcc0ed9","name":"Esme"}
```

```sh
curl -X GET http://localhost:3030/people/200
Person not found
```

* Update a person endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"id":"f1601fc5-f0c9-4950-8017-e094b284cad9", "name":"LuisFer"}' \
-X PUT http://localhost:3030/people

{"id":"f1601fc5-f0c9-4950-8017-e094b284cad9","name":"LuisFer"}
```

* Delete a person endpoint

```sh
curl -H "Content-Type: application/json" \
-X DELETE http://localhost:3030/people/d3bc8246-53da-4275-b833-5feb4489741d

Person d3bc8246-53da-4275-b833-5feb4489741d deleted
```

## Migration

I am using `sqlx-cli`, so let's install it first.

```sh
cargo install sqlx-cli
```

* add migration for people table

```sh
sqlx migrate add -r people_table

Creating migrations/20230917172957_people_table.up.sql
Creating migrations/20230917172957_people_table.down.sql
```

migration files were added in the `migrations` directory.

```sh
migrations/20230917172957_people_table.up.sql
migrations/20230917172957_people_table.down.sql
```

* add migration for pets table

```sh
sqlx migrate add -r pets_table

Creating migrations/20230917183452_pets_table.up.sql
Creating migrations/20230917183452_pets_table.down.sql
```

* run migrations

```sh
sqlx migrate run --database-url postgresql://localhost:5432/pipol
```

* revert migrations

Each revert will trigger the latest migration and try to run the `*.down.sql` script.

```sh
sqlx migrate revert --database-url "postgresql://localhost:5432/pipol"
```

## How to check database

* get into the database
```sh
psql -U pipol -h localhost -p 5432
```

* list tables
```sh
pipol=# \dt
        List of relations
 Schema |  Name  | Type  | Owner
--------+--------+-------+-------
 public | people | table | pipol
 public | pets   | table | pipol
(2 rows)
```