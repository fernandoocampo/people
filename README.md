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
⏱️	Starting people api application...
🪵	Initializing logger...
🗿	Starting database connection...
🛤️  Establishing API routes...
👤	Creating people endpoint: GET /people
👤	Creating get person endpoint: GET /people/{id}
👤	Creating update person endpoint: PUT /people/{id}
👤	Creating add person endpoint: POST /people
🍏	Server has started at :3030
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
--data '{"id":"3", "name":"Esme"}' \
-X POST http://localhost:3030/people

Person added
```

* Calling get people endpoint

```sh
curl -X GET http://localhost:3030/people
[{"id":"1","name":"Luis"},{"id":"2","name":"Fernando"}]
```

with params

```sh
curl -X GET 'http://localhost:3030/people?start=0&end=2'
[{"id":"1","name":"Luis"},{"id":"2","name":"Fernando"}]
```

* Calling get a person endpoint

```sh
curl -X GET http://localhost:3030/people/1
{"id":"1","name":"Luis"}
```

```sh
curl -X GET http://localhost:3030/people/200
Person not found
```

* Update a person endpoint

```sh
curl -H "Content-Type: application/json" \
--data '{"id":"1", "name":"LuisFer"}' \
-X PUT http://localhost:3030/people/1

Person updated
```

* Delete a person endpoint

```sh
curl -H "Content-Type: application/json" \
-X DELETE http://localhost:3030/people/2

Person deleted
```