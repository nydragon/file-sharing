use std::net::IpAddr;

mod connection_info;
mod serve_file;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to share
    #[arg(short, long)]
    filename: String,
}

fn main() {
    use local_ip_address::local_ip;

    let args: Args = Args::parse();

    let ip: IpAddr = local_ip().unwrap();
    let port: u16 = 8080;
    let file: String = String::from(args.filename);

    connection_info::display(ip, port);

    serve_file::serve_file(ip, port, file);
}
