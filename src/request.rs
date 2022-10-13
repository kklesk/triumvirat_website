// use super::method::Method;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display, write};
use std::error::Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: super::method::Method,
}
//TODO
impl TryFrom<&[u8]> for Request{

    // type Error = ();
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
        todo!()
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
// impl Error for ParseError{
//
// }
//FIXME
// impl Display for ParseError{
//     fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result{
//         write!(f,"{}",self.message())
//     }
// }

impl ParseError{
    fn message(&self) -> &str{
        match self{
            self::ParseError::InvalidRequest => "InvalidMethod",
            self::ParseError::InvalidEncoding=> "InvalidEncoding",
            self::ParseError::InvalidProtocol=> "InvalidProtocol",
            self::ParseError::InvalidMethod => "InvalidMethod",
        }
    }
}

//FIXME
// impl Display for Request {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f,"{}",self)
//     }
// }
//
