use std::net::IpAddr;

pub fn create_qr_code(ip: IpAddr, port: u16) {
    use qrcode_generator::QrCodeEcc;

    let full_address: String = format!("{}:{}/download", ip, port);

    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix(full_address, QrCodeEcc::Low).unwrap();

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
