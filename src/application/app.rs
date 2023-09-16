use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::json::JsonEncoder;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

use crate::errors::error;
use crate::people;
use crate::storage::db;

pub async fn run() {
    println!("🪵\tInitializing logger...");
    initialize_logger();

    log::info!("🗿\tStarting database connection...");
    let store = new_db_storage().await;
    let store_filter = warp::any().map(move || store.clone());

    log::info!("🪜 \tEstablishing API routes...");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    log::info!("👤\tCreating people endpoint: GET /people");
    let get_people = warp::get()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(people::handler::get_people)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_people request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    log::info!("👤\tCreating get person endpoint: GET /people/{{id}}");
    let get_person = warp::get()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(people::handler::get_person);

    log::info!("👤\tCreating update person endpoint: PUT /people");
    let put_person = warp::put()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(people::handler::update_person);

    log::info!("👤\tCreating add person endpoint: POST /people");
    let post_person = warp::post()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(people::handler::add_person);

    log::info!("👤\tCreating delete person endpoint: DELETE /people/{{id}}");
    let delete_person = warp::delete()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(people::handler::delete_person);

    // let wrap_log = warp::log::custom(|info| {
    //     log::info!(
    //         "{} {} {} {:?} from {} with {:?}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
    //         info.elapsed(),
    //         info.remote_addr().unwrap(),
    //         info.request_headers(),
    //     );
    // });

    let routes = get_people
        .or(get_person)
        .or(put_person)
        .or(post_person)
        .or(delete_person)
        .with(cors)
        .with(warp::trace::request())
        .recover(error::return_error);

    log::info!("🍏\tStarting server at :3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn initialize_logger() {
    let log_system = env::var("LOG_SYSTEM");

    match log_system {
        Ok(log_system) => init_specific_logger(&log_system),
        Err(_) => {
            eprintln!("LOG_SYSTEM: log4rs");
            initialize_log4rs()
        }
    }
}

fn init_specific_logger(log_system: &str) {
    eprintln!("LOG_SYSTEM: {}", log_system);
    match log_system {
        "tracing" => initialize_tracing(),
        "envlogger" => initialize_env_logger(),
        "log4rs" => initialize_log4rs(),
        _ => initialize_log4rs(),
    }
}

fn initialize_env_logger() {
    env_logger::init();
    log::info!("🪵\tUsing env_logger");
}

fn initialize_log4rs() {
    let app_stdout = ConsoleAppender::builder().build();

    let people_stdout = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("app_stdout", Box::new(app_stdout)))
        .appender(Appender::builder().build("people_stdout", Box::new(people_stdout)))
        // .logger(Logger::builder().appender("app_stdout").build("people::application::app", LevelFilter::Debug))
        .logger(
            Logger::builder()
                .appender("people_stdout")
                .build("warp::*", LevelFilter::Debug),
        )
        .logger(
            Logger::builder()
                .appender("people_stdout")
                .build("hyper::*", LevelFilter::Debug),
        )
        .logger(
            Logger::builder()
                .appender("people_stdout")
                .build("people::people::handler", LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("app_stdout")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
    log::info!("🪵\tUsing log4rs");
}

fn initialize_tracing() {
    tracing_subscriber::fmt()
        // determine which traces to record
        .with_env_filter(new_tracing_log_filter())
        // record an event when each span closes
        // used to time our routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        // activate subscriber
        .init();
}

fn new_tracing_log_filter() -> String {
    env::var("RUST_LOG").unwrap_or_else(|_| "people=debug,warp=error".to_owned())
}

async fn new_db_storage() -> db::Store {
    let db_url = "postgres://pipol:pipol@localhost:5432/pipol";

    db::Store::new(db_url).await
}
