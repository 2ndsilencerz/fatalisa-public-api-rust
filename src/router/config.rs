use crate::route_list;
use crate::service;
use warp::Filter;

pub async fn run() {
    // initialize routes
    let index_route = route_list::index_router;
    let post_route = warp::post().map(|| service::index::index());

    // add routes
    let routes = index_route().or(post_route);

    // run webservice
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}