use std::net::IpAddr;

/// Generate a QR code and displays it to the terminal
fn create_qr_code(url: &String) {
    use qrcode_generator::QrCodeEcc;

    let result: Vec<Vec<bool>> =
        qrcode_generator::to_matrix(url, QrCodeEcc::Low).expect("Couldn't generate QR code.");

    for line in result {
        for bit in line {
            print!("{}", if bit { "██" } else { "  " });
        }
        print!("\n")
    }
}

/// Displays a URL to the file download and its corresponding QR code.
pub fn display(ip: IpAddr, port: u16) {
    let url: String = format!("http://{}:{}/download", ip, port);

    create_qr_code(&url);

    println!("\nConnect to: {}", url);
}
