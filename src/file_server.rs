use std::fs;
use super::server::HttpRequestHandler;
use super::http::StatusCode;
use super::http::Request;
use super::http::Response;
use super::http::Method;

/// The FileServer is responsible for handling HTTP requests and, when appropriate, returning the contents of files from a `public` directory
pub struct FileServer {
    /// Absolute path to a directory where all files served by us are stored
    public_path: String
}

impl FileServer {
    /// Returns a new FileServer
    /// # Arguments
    /// * `public_path` - The path of the directory to serve files from
    pub fn new(public_path: String) -> Self {
        Self{public_path}
    }

    /// Attempts to read a file from the `public` directory
    /// # Arguments
    /// * `file_path` - The *relative* path to a file
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // We canonicalize the path for 2 reasons
        // 1. If the path does not exist, it will error
        // 2. It removes `..`, giving us a path that *should*
        //    start with our `public` directory path
        match fs::canonicalize(path) {
            Ok(path) => {
                // A valid path will have our `public` directory
                // as its root. Any other paths are illegal and
                // must be rejected for security.
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Malicious file path attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl HttpRequestHandler for FileServer {
    /// Given a HTTP request, returns a HTTP response.None
    /// # Supported request types
    /// * GET - will attempt to return a response containing the contents of a local file (e.g. .html, .css, .js and other static resources)
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            // Route requests with path `/` to index.html, all
            // other paths are attempted to be read from our
            // `public` directory
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}