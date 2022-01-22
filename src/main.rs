#![allow(dead_code)]

mod http;
mod server;
mod file_server;

use server::Server;
use file_server::FileServer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // This is the IP and port for the server to listen to
    let default_server_address = "127.0.0.1:8080".to_string();
    let server_address = args.get(1).unwrap_or(&default_server_address).to_string();

    // This is the path to where the static website files are stored (e.g. .html, .css, .js)
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);

    let server = Server::new(server_address);
    server.run(FileServer::new(public_path));
}
