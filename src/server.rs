use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;
use crate::http::Response;
use crate::http::StatusCode;
use crate::http::ParseError;

/// Handles methods for parsed HTTP requests
pub trait HttpRequestHandler {
    /// Handler for any requests that were successfully parsed
    fn handle_request(&mut self, request: &Request) -> Response;

    /// Handler for any requests that we failed to parse
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

/// HTTP Server responsible for handling incoming HTTP requests and serving responses
pub struct Server {
    /// Address string composed of IP and port
    address: String,
}

impl Server {
    /// Returns a new server
    /// # Arguments
    /// * `address` - Address string in the form {ip}:{port}
    pub fn new(address: String) -> Self {
        Self { address: address }
    }

    /// Starts the server listening indefinitely for incoming HTTP requests
    /// # Arguments
    /// * `handler` - HTTP request handler to forward *parsed* requests onto
    pub fn run(self, mut handler: impl HttpRequestHandler) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            let result = listener.accept();
            match result {
                Ok((mut stream, _)) => {
                    // Buffer size of 1KB would not be large enough
                    // for a real HTTP server, but acceptable for our uses
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error)
                            };
                            if let Err(error) = response.send(&mut stream) {
                                println!("Failed to send response: {}", error);
                            }
                        }
                        Err(error) => println!("Failed to read from stream: {}", error),
                    }
                }
                Err(error) => println!("Failed to accept connection: {}", error),
            }
        }
    }
}
