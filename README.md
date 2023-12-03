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

* Register a User endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"email": "esme@anydomain.com", "password": "123456"}' \
-X POST http://localhost:3030/signup

{"id":"98bd8597-1ead-4cc3-adfc-453441b3002a"}
```

* login User endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"email": "esme@anydomain.com", "password": "123456"}' \
-X POST http://localhost:3030/login

"v2.local.RU8RaG9_YX_f1rXCSIDOfryxkrItp6A5cE8FeZlHVzAiswFHU7N0EYyvUbpkQIbVPD5meuxA6a0ZyDJT_Hnv3pp685U8Hm0Z38BYgPBMuBbcd3Tn0bg2eBHEoBj-L0TcOwhOKD_UwzSQEFq07cI9EIdoo1HiGejRBblYQuqsRhZJWBmKaOk3pG01hadgUzvLMauKYg6RdazW8o9zyFX17IAojeQehaHNl0CSN95DGhFALxhTbdCpbBK0aG4JVmWKXDoYC5GFUD-IkkV02wAx1BQDBjTO4-8cYpjCRzxuBpFjE4mIS7FenPJ3T4s-QS4herES3gLDzpfRtlQ8Du6nGzZYD9GGF0_0bK5BZSk.a2V5LWlkOmdhbmRhbGYw"
```

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
[{"id":"27c6bdd9-67d6-4503-884d-d75aba152f44","first_name":"Esme","last_name":"Esme"},{"id":"d49aed14-b5b0-4e49-972f-f823817ed93d","first_name":"Fernando","last_name":"Fernando"}]
```

with params

limit: The index of the last item which has to be returned
offset: The index of the first item which has to be returned

```sh
curl -X GET 'http://localhost:3030/people?limit=10&offset=0'

[{"id":"27c6bdd9-67d6-4503-884d-d75aba152f44","first_name":"Esme","last_name":"Esme"},{"id":"d49aed14-b5b0-4e49-972f-f823817ed93d","first_name":"Fernando","last_name":"Fernando"}]
```

* Calling get a person endpoint

```sh
curl -X GET http://localhost:3030/people/27c6bdd9-67d6-4503-884d-d75aba152f44

{"id":"27c6bdd9-67d6-4503-884d-d75aba152f44","first_name":"Esme","last_name":"Esme"}
```

```sh
curl -X GET http://localhost:3030/people/200
Person not found
```

* Update a person endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"id":"27c6bdd9-67d6-4503-884d-d75aba152f44", "first_name":"Esme", "last_name":"Emse"}' \
-X PUT http://localhost:3030/people

{"id":"27c6bdd9-67d6-4503-884d-d75aba152f44","first_name":"Esme","last_name":"Emse"}
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