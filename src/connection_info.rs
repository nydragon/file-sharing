use std::net::IpAddr;

fn create_qr_code(url: &String) {
    use qrcode_generator::QrCodeEcc;

    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix(url, QrCodeEcc::Low).unwrap();

    for line in result {
        for bit in line {
            print!("{}", if bit { "██" } else { "  " });
        }
        print!("\n")
    }
}

pub fn display(ip: IpAddr, port: u16) {
    let url: String = format!("http://{}:{}/download", ip, port);

    create_qr_code(&url);

    println!("\nConnect to: {}", url);
}
