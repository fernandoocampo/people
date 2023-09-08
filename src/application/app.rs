use warp::{http::Method, Filter};

use crate::errors::error;
use crate::people;
use crate::storage::memory::Store;

pub async fn run() {
    println!("🗿\tStarting database connection...");
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    println!("🛤️\tEstablishing API routes...");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    println!("👤\tCreating people endpoint: GET /people");
    let get_people = warp::get()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(people::handler::get_people);

    println!("👤\tCreating get person endpoint: GET /people/{{id}}");
    let get_person = warp::get()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(people::handler::get_person);

    println!("👤\tCreating update person endpoint: PUT /people/{{id}}");
    let put_person = warp::put()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(people::handler::update_person);

    println!("👤\tCreating add person endpoint: POST /people");
    let post_person = warp::post()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(people::handler::add_person);

    let routes = get_people
        .or(get_person)
        .or(put_person)
        .or(post_person)
        .with(cors)
        .recover(error::return_error);

    println!("🍏\tServer has started at :3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
