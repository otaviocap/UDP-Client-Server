use std::io::BufRead;
use std::net::UdpSocket;
use std::time::Duration;

pub fn run_client(ip: &str, buffer_size: usize) {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind to ip and port");

    let one_second = Some(Duration::from_secs(1));
    socket
        .set_read_timeout(one_second)
        .expect("Could not set read timeout");
    socket
        .set_write_timeout(one_second)
        .expect("Could not set write timeout");

    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Something went wrong while getting stdin line");

        socket
            .send_to(line.as_bytes(), ip.to_string())
            .expect("Something went wrong while sending the data");

        let mut buffer = vec![0; buffer_size];

        let (data_received, src_address) = socket
            .recv_from(&mut buffer)
            .expect("Something went wrong while receiving the data");

        let buffer = &buffer[..data_received];

        println!(
            "Received data from {src_address}, data: {}",
            std::str::from_utf8(buffer).expect("Could not parse to UTF-8")
        );

        if &line == "exit" {
            break;
        }
    }
}
