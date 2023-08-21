mod service;
mod router;

use crate::router::route_list;

#[tokio::main]
async fn main() {
    router::config::run().await;
}