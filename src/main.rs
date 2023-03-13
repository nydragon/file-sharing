use local_ip_address::local_ip;
use std::net::IpAddr;

mod qr_code;
mod serve_file;

fn main() {
    // implement flag handling to get the file to send
    let ip: IpAddr = local_ip().unwrap();
    let port: u16 = 8080;
    let file: String = String::from("src/main.rs");

    qr_code::create_qr_code(ip, port);

    serve_file::serve_file(ip, port, file);
}
