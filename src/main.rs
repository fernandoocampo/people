#![warn(clippy::all)]

pub mod application;
pub mod censors;
pub mod errors;
pub mod people;
pub mod storage;
pub mod types;
pub mod users;

#[tokio::main]
async fn main() {
    println!("⏱️\tStarting people api application...");
    application::app::run().await;
}
