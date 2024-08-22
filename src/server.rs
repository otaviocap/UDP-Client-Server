use crate::file_server::mocked::FILE_SERVER;
use crate::message::Message;
use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn run_server(ip: &str, buffer_size: usize) {
    let socket = UdpSocket::bind(ip).expect("Could not bind to ip and port");
    println!("Server bound to: {ip}");

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    receive_routine(&socket, receiver);

    send_routine(buffer_size, socket, sender);
}

fn send_routine(buffer_size: usize, socket: UdpSocket, sender: Sender<Message>) {
    loop {
        let mut buffer = vec![0; buffer_size];

        let (data_received, src_address) = socket
            .recv_from(&mut buffer)
            .expect("Something went wrong while receiving the data");

        let buffer = &buffer[..data_received];
        let buffer_msg = std::str::from_utf8(buffer).expect("Could not parse to UTF-8");

        println!("Received data from {src_address}, data: {}", buffer_msg);

        let processed_data = process_data(buffer_msg);

        sender
            .send(Message {
                address: src_address.to_string(),
                data: processed_data.to_vec(),
            })
            .expect("Something went wrong while sending the data to sender thread")
    }
}

fn process_data(data: &str) -> Vec<u8> {
    if data.starts_with("file:") {
        let path = data
            .split_terminator(":")
            .last()
            .unwrap()
            .trim()
            .split_terminator('/')
            .map(|e| e.to_string())
            .collect();

        return match FILE_SERVER.get(path) {
            None => "File not found".as_bytes().to_vec(),
            Some(data) => data.as_bytes().to_vec(),
        };
    }

    (data.to_ascii_uppercase() + " (hehe to upper 🤖)")
        .as_bytes()
        .to_vec()
}

fn receive_routine(socket: &UdpSocket, receiver: Arc<Mutex<Receiver<Message>>>) {
    let receiver_socket = socket
        .try_clone()
        .expect("Something went wrong while cloning socket");

    thread::spawn(move || loop {
        let message: Message = receiver
            .lock()
            .expect("Something went wrong while locking the receiver mutex")
            .recv()
            .expect("Something went wrong while receiving an message from sender thread");

        println!(
            "Sending to {}, data: {}",
            message.address,
            std::str::from_utf8(message.data.as_slice()).expect("Could not parse to UTF-8")
        );

        receiver_socket
            .send_to(message.data.as_slice(), message.address)
            .expect("Something went wrong while sending the data");
    });
}
