mod service;
mod router;

use warp::Filter;
use crate::router::route_list;

#[tokio::main]
async fn main() {
    web_init().await
}

async fn web_init() {
    // initialize routes
    let index_route = route_list::index_router;
    let post_route = warp::post().map(|| service::index::index());

    // add routes
    let routes = index_route().or(post_route);

    // run webservice
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}