use warp::{Filter, Rejection};
use crate::service::index;

pub fn index_router() -> impl Filter<Extract = (&'static str,), Error = Rejection> + Clone {
    warp::get().map(|| index::index())
}