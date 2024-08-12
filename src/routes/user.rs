use warp::{Filter, Rejection, Reply};

// GET /dashboard
fn current_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).map(|| "msneed")
}

// GET /dashboard/age
fn current_user_age() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("age")
        .and(warp::get())
        .map(|| format!("User age: {}", 323))
}

// Returns all routes for Dashboard
pub fn dashboard_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("dashboard").and(current_user().or(current_user_age()))
}
