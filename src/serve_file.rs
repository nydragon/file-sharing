use tokio;
use warp::{
    http::HeaderValue,
    hyper::{HeaderMap, StatusCode},
    Filter, Rejection, Reply,
};

use std::path::Path;
use std::{convert::Infallible, net::IpAddr};

#[tokio::main]
pub async fn serve_file(ip: IpAddr, port: u16, file: String) {
    let headers: HeaderMap = generate_header(&file);

    let download_route = warp::path("download")
        .and(warp::fs::file(file))
        .with(warp::reply::with::headers(headers));

    let router = download_route.recover(handle_rejection);

    warp::serve(router).run((ip, port)).await;
}

fn generate_header(file: &String) -> HeaderMap {
    let file: &Path = Path::new(&file);
    let mut headers: HeaderMap = HeaderMap::new();

    if let Some(filename) = file.file_name() {
        if let Some(filename) = filename.to_str() {
            let val = format!("attachement; filename = \"{}\"", filename);
            if let Ok(d) = HeaderValue::from_str(val.as_str()) {
                headers.insert("Content-Disposition", d);
            };
        }
    }

    headers
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
