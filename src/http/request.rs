use std::convert::From;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::Utf8Error;

use super::Method as Method;
use super::MethodError as MethodError;
use super::QueryString;

/// Represents a HTTP request (simplified!)
#[derive(Debug)]
pub struct Request<'buf> {
    /// Method type, e.g. `GET`
    method: Method,
    /// Requested path, e.g. `/index.html`
    path: &'buf str,
    /// Optional query parameters
    query: Option<QueryString<'buf>>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    /// Attempts to parse and return a request from a buffer
    /// 
    /// # Arguments
    /// * `buf` - The buffer to read the request from
    /// 
    /// # Returns
    /// * The parsed request or a `ParseError`
    /// 
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path_and_query, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query = None;
        let mut path = &path_and_query[..];

        if let Some(idx) = path_and_query.find('?') {
            query = Some(QueryString::from(&path_and_query[idx + 1..]));
            path = &path_and_query[..idx];
        }

        Ok(Self {
            method: method,
            path: path,
            query,
        })
    }
}

/// For a string, attempts to find the next 'word'
/// delineated by a space or newline character.
/// 
/// # Arguments
/// * `text` - the string to search for the next word
/// 
/// # Returns
/// * The next word and the remaining text after the word or `None`
/// 
fn get_next_word(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }
    return None;
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
