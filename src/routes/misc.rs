use warp::{Filter, Rejection, Reply};

// GET /
pub fn index_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().map(|| "Index page")
}

// GET /hello
pub fn hello() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("hello")
        .and(warp::get())
        .map(|| format!("This is a test!"))
}
