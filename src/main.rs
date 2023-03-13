use local_ip_address::local_ip;
use qrcode_generator::QrCodeEcc;
use std::path::Path;
use std::{convert::Infallible, net::IpAddr};
use tokio;
use warp::{
    http::HeaderValue,
    hyper::{HeaderMap, StatusCode},
    Filter, Rejection, Reply,
};

fn main() {
    // implement flag handling to get the file to send

    let ip: IpAddr = local_ip().unwrap();
    let port: u16 = 8080;
    let file: String = String::from("src/main.rs");

    create_qr_code(ip, port);

    serve_file(ip, port, file);
}

fn create_qr_code(ip: IpAddr, port: u16) {
    let full_address = format!("{}:{}/download", ip, port);

    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix(full_address, QrCodeEcc::Low).unwrap();

    println!("{:?}", result);

    for line in result {
        for bit in line {
            if bit == true {
                print!("██")
            } else {
                print!("  ")
            }
        }
        print!("\n")
    }
}

#[tokio::main]
async fn serve_file(ip: IpAddr, port: u16, file: String) {
    let headers: HeaderMap = generate_header(&file);

    let download_route = warp::path("download")
        .and(warp::fs::file(file))
        .with(warp::reply::with::headers(headers));

    let router = download_route.recover(handle_rejection);

    warp::serve(router).run((ip, port)).await;
}

fn generate_header(file: &String) -> HeaderMap {
    let file = Path::new(&file);
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
