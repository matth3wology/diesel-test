use misc::{hello, index_filter};
use user::dashboard_routes;
use warp::{Filter, Rejection, Reply};

mod misc;
mod user;

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index_filter().or(hello()).or(dashboard_routes())
}
