use crate::models::NewUrl;
use crate::service;
use handlebars::Handlebars;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::{Rejection, Reply};

pub async fn show_form<'a>(handlebars: Arc<Handlebars<'a>>) -> Result<impl Reply, Rejection> {
    let data: HashMap<String, String> = HashMap::new();
    let rendered = handlebars.render("form", &data).unwrap();

    Ok(warp::reply::html(rendered))
}

pub async fn create_short_url<'a>(
    form: NewUrl,
    db: Arc<Mutex<Connection>>,
    handlebars: Arc<Handlebars<'a>>,
) -> Result<impl Reply, Rejection> {
    let response = service::create_url(form, db).unwrap();

    let mut data = HashMap::new();
    data.insert("message", response);

    let rendered = handlebars.render("form", &data).unwrap();

    Ok(warp::reply::html(rendered))
}
