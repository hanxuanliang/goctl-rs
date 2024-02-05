#![allow(dead_code)]

use crate::common::{IResult, Input};

#[derive(Debug)]
pub struct PError(pub String);

impl PError {
    pub fn from<O>(msg: &str) -> IResult<O> {
        Err(nom::Err::Error(PError(msg.to_string())))
    }
}

impl nom::error::ParseError<Input<'_>> for PError {
    fn from_error_kind(input: Input, kind: nom::error::ErrorKind) -> Self {
        PError(format!("Error: {:?} at {:?}", kind, input))
    }

    fn append(_: Input, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
