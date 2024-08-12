extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use warp::Filter;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "db.sqlite";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let pool_filter = warp::any().map(move || pool.clone());

    let list_users = warp::path("users")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(list_users_handler);

    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(create_user_handler);

    let routes = list_users.or(create_user);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn list_users_handler(pool: DbPool) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::users::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection.");
    let results = users
        .load::<models::User>(&mut conn)
        .expect("Error loading users");

    Ok(warp::reply::json(&results))
}

async fn create_user_handler(
    new_user: models::NewUser,
    pool: DbPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::users;

    let mut conn = pool.get().expect("Failed to get DB connection.");
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");

    Ok(warp::reply::with_status(
        "User created",
        warp::http::StatusCode::CREATED,
    ))
}
