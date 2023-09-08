use warp::{http::Method, Filter};

use crate::errors::error;
use crate::people;
use crate::storage::memory::Store;

pub async fn run() {
    println!("🗿\tStarting database connection...");
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    println!("🛤️\tEstablishing API routes...");

    println!("👤\tCreating people endpoint: GET /people");
    let get_people = warp::get()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(people::handler::get_people);

    let routes = get_people.with(cors).recover(error::return_error);

    println!("🍏\tServer has started at :3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
