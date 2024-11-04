mod db;
mod handlers;
mod models;
mod service;

use handlebars::Handlebars;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(db::init_db()));
    let handlebars = Arc::new({
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("form", "templates/form.hbs")
            .expect("Template error");
        handlebars
            .register_template_file("success", "templates/success.hbs")
            .expect("Template error");
        handlebars
    });

    let form_route = warp::path::end()
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(handlers::show_form);

    let create_route = warp::path("create")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_db(db.clone()))
        .and(with_handlebars(handlebars.clone()))
        .and_then(handlers::create_short_url);

    let routes = form_route.or(create_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(
    db: Arc<Mutex<Connection>>,
) -> impl Filter<Extract = (Arc<Mutex<Connection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_handlebars(
    handlebars: Arc<Handlebars>,
) -> impl Filter<Extract = (Arc<Handlebars>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || handlebars.clone())
}
