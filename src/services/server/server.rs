use warp::{Filter, Rejection, Reply};
use std::convert::Infallible;

pub async fn start_server() {
    // GET
    let get_route = warp::get()
        .and(warp::path::full())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|path: warp::path::FullPath, params: std::collections::HashMap<String, String>| {
            println!("Received GET request:");
            println!("Path: {}", path.as_str());
            println!("Query parameters: {:?}", params);
            format!("Received GET request on path: {}", path.as_str())
        });

    // POST
    let post_route = warp::post()
        .and(warp::path::full())
        .and(warp::body::json())
        .map(|path: warp::path::FullPath, body: serde_json::Value| {
            println!("Received POST request:");
            println!("Path: {}", path.as_str());
            println!("Body: {:?}", body);
            format!("Received POST request on path: {}", path.as_str())
        });

    let routes = get_route.or(post_route);

    let routes = routes.recover(rejection_handler);

    println!("Server starting on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub async fn rejection_handler(err: Rejection) -> Result<impl Reply, Infallible> {
    println!("An error occurred: {:?}", err);
    Ok(warp::reply::with_status(
        "An error occurred",
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
