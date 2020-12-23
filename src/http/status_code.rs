use std::fmt::{Display, Formatter, Result as fmtResult};

/*
We will only implement a few statuscodes as implementing all of them would not serve the educative purpose of this project.
*/

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found", 
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{}", *self as u16)
    }
}