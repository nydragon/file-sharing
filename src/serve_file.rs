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

/// Returns the name of the file pointed to by a path
fn get_file_name(file: &String) -> Option<String> {
    let file: &Path = Path::new(&file);

    let filename = file.file_name()?;

    let filename = filename.to_str()?;

    Some(filename.to_string())
}

/// Generate the response header that instructs the client to download the served file
fn generate_header(file: &String) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();

    let filename: String = get_file_name(file).expect("Couldn't get file name from path.");

    let val: String = format!("attachement; filename=\"{}\"", filename);

    if let Ok(header_value) = HeaderValue::from_str(val.as_str()) {
        headers.insert("Content-Disposition", header_value);
    };

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
