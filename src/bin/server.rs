use std::{env, process};

use tcp_web::HttpHandler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("example: {} <ip_address> <port>", args[0]);
        process::exit(1);
    }

    let mut handler = HttpHandler::new();
    handler.ip_address(&args[1]);
    handler.port(args[2].parse().unwrap());

    handler.start();
}
