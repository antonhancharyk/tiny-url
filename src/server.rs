use std::convert::Infallible;
use std::fs::read_to_string;
use warp::Filter;

#[tokio::main]
pub async fn run() {
    let html_route = warp::path::end().and_then(serve_html);

    warp::serve(html_route).run(([127, 0, 0, 1], 8080)).await;
}

async fn serve_html() -> Result<impl warp::Reply, Infallible> {
    match read_to_string("static/index.html") {
        Ok(content) => Ok(warp::reply::html(content)),
        Err(_) => Ok(warp::reply::html(
            "<h1>404 - File not found</h1>".to_string(),
        )),
    }
}

use super::db::{Database, User};
use warp::http::StatusCode;
use warp::reply::Json;
use warp::{Rejection, Reply};

// Handler to add a new user
pub async fn add_user(db: Database, new_user: User) -> Result<impl Reply, Rejection> {
    match db.add_user(new_user) {
        Ok(_) => Ok(warp::reply::with_status(
            "User added successfully",
            StatusCode::CREATED,
        )),
        Err(err) => Ok(warp::reply::with_status(
            format!("Failed to add user: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

// Handler to get all users
pub async fn get_users(db: Database) -> Result<Json, Rejection> {
    match db.get_users() {
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(err) => Ok(warp::reply::with_status(
            format!("Failed to fetch users: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
