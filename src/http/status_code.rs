use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

/// HTTP status codes (we only implement a small subset of them)
/// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status>
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}