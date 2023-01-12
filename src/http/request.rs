use super::method::{Method, MethodError};
use super::QueryString;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{Result as FmtResult, Formatter, Debug};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request <'buff>{
    path : &'buff str,
    query_string : Option<QueryString<'buff>>,
    method: Method
}

// impl <'buff> Request <'buff> {
//     pub fn path(&self) -> &str {
//         &self.path
//     }
// }


impl <'buff> Request<'buff> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}


impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}


impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }

}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }

}

impl Display for ParseError{
    fn fmt(&self, f : &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f : &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl <'buff> Request <'buff>{
    pub fn from_byte_array(buf : &[u8]) -> Result<Self, String> {
        unimplemented!() // to silence compiler
    }
}

impl <'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;
    fn try_from(buf: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {
 
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method : Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1 ..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })

        // unimplemented!()  // to silence compiler
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate() {

        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
    // unimplemented!()
}
