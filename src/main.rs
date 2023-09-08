pub mod application;
pub mod errors;
pub mod people;
pub mod storage;
pub mod types;

#[tokio::main]
async fn main() {
    println!("⏱️\tStarting people api application...");
    application::app::run().await;
}
