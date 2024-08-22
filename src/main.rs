mod client;
mod file_server;
mod message;
mod server;

use crate::client::run_client;
use crate::server::run_server;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let buffer_size = 2048;

    if args.len() < 2 {
        println!("Usage: << ip:port >> << server / client >>");
        println!("Example: 127.0.0.1:12345 server");
        std::process::exit(1);
    }

    let hostname = args
        .get(1)
        .expect("Something went wrong while getting the second argument from the command");

    match args[2].as_str() {
        "client" => run_client(hostname, buffer_size),
        "server" => run_server(hostname, buffer_size),
        _ => panic!("Error, invalid option. Valid options: server, client"),
    }
}
