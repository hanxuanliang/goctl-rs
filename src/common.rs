use crate::{
    error::{self, PError},
    token::{APIToken, APITokenKind},
};
use nom::Slice;

pub type Input<'a> = &'a [APIToken<'a>];
pub type IResult<'a, Output> = nom::IResult<Input<'a>, Output, error::PError>;

pub fn match_token(kind: APITokenKind) -> impl Fn(Input) -> IResult<&APIToken> {
    move |i: Input| match i.get(0) {
        Some(token) if token.kind == kind => Ok((i.slice(1..), token)),
        Some(token) => Err(nom::Err::Error(PError(format!(
            "Expected API Token {kind}, found {} at {}",
            token.kind, token.at
        )))),
        _ => Err(nom::Err::Error(PError(format!(
            "API Token {:?} does not match",
            kind
        )))),
    }
}

pub fn match_text(text: &'static str) -> impl Fn(Input) -> IResult<&APIToken> {
    move |i| match i.get(0).filter(|token| token.text() == text) {
        Some(token) => Ok((i.slice(1..), token)),
        None => Err(nom::Err::Error(PError(format!(
            "Json Text {text} does not match",
        )))),
    }
}

pub fn match_text_case_insensitive(text: &'static str) -> impl Fn(Input) -> IResult<&APIToken> {
    move |i| match i
        .get(0)
        .filter(|token| token.text().eq_ignore_ascii_case(text))
    {
        Some(token) => Ok((i.slice(1..), token)),
        None => Err(nom::Err::Error(PError(format!(
            "Json Text {text} does not match",
        )))),
    }
}
