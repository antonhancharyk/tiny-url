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
