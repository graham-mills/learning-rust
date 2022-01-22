use std::fmt::Debug;
use std::io::Write;
use std::io::Result as IoResult;

use super::StatusCode;

/// Represents a HTTP response (simplified!)
#[derive(Debug)]
pub struct Response {
    /// Status code, e.g. 404 Not Found
    status_code: StatusCode,
    /// Optional body
    body: Option<String>
}

impl Response {
    /// Returns a new `Response`
    /// 
    /// # Arguments
    /// * `status_code` - The HTTP status code of the response
    /// * `body` - An optional body to return (e.g. file contents)
    /// 
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }

    /// Writes the response to a stream as a raw string
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phase(), 
            body
        )
    }
}