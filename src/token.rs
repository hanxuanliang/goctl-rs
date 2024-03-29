#![allow(dead_code)]

use std::ops::Range;

use logos::{Lexer, Logos};

pub struct APITokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, APITokenKind>,
}

impl<'a> APITokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: APITokenKind::lexer(source),
        }
    }
}

impl<'a> Iterator for APITokenizer<'a> {
    type Item = APIToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Ok(kind)) => Some(APIToken {
                source: self.source,
                kind,
                at: self.lexer.slice(),
                span: self.lexer.span(),
            }),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub struct APIToken<'a> {
    pub source: &'a str,
    pub kind: APITokenKind,
    pub at: &'a str,
    pub span: Range<usize>,
}

impl<'a> APIToken<'a> {
    pub fn text(&self) -> &'a str {
        &self.source[self.span.clone()]
    }
}

impl std::fmt::Debug for APIToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}[{:?}] @ {}..{}",
            self.kind,
            self.text(),
            self.span.start,
            self.span.end
        )
    }
}

pub fn tokenize(source: &str) -> Vec<APIToken> {
    APITokenizer::new(source).collect::<Vec<_>>()
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum APITokenKind {
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    // basic tokens
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token(":")]
    Colon,

    // data types
    #[regex("int(8|16|32|64)?")]
    IntDataType,
    #[regex("float(32|64)")]
    FloatDataType,
    #[regex("string")]
    StringDataType,
    #[regex("bool")]
    BoolDataType,
    #[token("map")]
    MapDataType,

    // type modifiers
    #[token("type")]
    Type,
    // #[regex(r#"[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    #[regex(r"[_a-zA-Z][_$a-zA-Z0-9]*|[0-9]+[_$a-zA-Z0-9]*")]
    Identifier,
    #[token("struct")]
    Struct,
    // #[regex(r#"`[a-zA-Z0-9_]+:"[^"]+(?:,[^"]+)*"`"#)]
    #[regex(r#"`(?:[a-zA-Z0-9_]+:"[^"]*"(?:\s+)?)+`"#)]
    TagAnnotation,

    // service modifiers
    #[token("@server")]
    Server,
    #[token("service")]
    Service,
    #[token("@handler")]
    Handler,
    #[regex("get|post")]
    HttpMethod,
    #[regex(r#"/([a-zA-Z0-9_]+/?)*"#)]
    RoutePath,
    #[token("returns")]
    RespReturns,
}

impl std::fmt::Display for APITokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APITokenKind::Whitespace => write!(f, "Whitespace"),
            APITokenKind::OpenBrace => write!(f, "OpenBrace"),
            APITokenKind::CloseBrace => write!(f, "CloseBrace"),
            APITokenKind::OpenParen => write!(f, "OpenParen"),
            APITokenKind::CloseParen => write!(f, "CloseParen"),
            APITokenKind::OpenBracket => write!(f, "OpenBracket"),
            APITokenKind::CloseBracket => write!(f, "CloseBracket"),
            APITokenKind::Colon => write!(f, "Colon"),
            APITokenKind::IntDataType => write!(f, "IntDataType"),
            APITokenKind::FloatDataType => write!(f, "FloatDataType"),
            APITokenKind::StringDataType => write!(f, "StringDataType"),
            APITokenKind::BoolDataType => write!(f, "BoolDataType"),
            APITokenKind::MapDataType => write!(f, "MapDataType"),
            APITokenKind::Type => write!(f, "Type"),
            APITokenKind::Identifier => write!(f, "Identifier"),
            APITokenKind::Struct => write!(f, "Struct"),
            APITokenKind::TagAnnotation => write!(f, "TagAnnotation"),
            APITokenKind::Server => write!(f, "Server"),
            APITokenKind::Service => write!(f, "Service"),
            APITokenKind::Handler => write!(f, "Handler"),
            APITokenKind::HttpMethod => write!(f, "HttpMethod"),
            APITokenKind::RoutePath => write!(f, "RoutePath"),
            APITokenKind::RespReturns => write!(f, "RespReturns"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizer() {
        let source = r#"
            type GetFormReq struct {
                Name    string   `form:"name,omitempty"`
                Age     int64    `form:"age" json:"age"`
                Hobbits []string `form:"hobbits"`
            }
            type GetFormResp struct {
                Total int64 `json:"total"`
            }

            @server (
                group:   json
	            jwt:     Auth
	            timeout: 3m
            )
            service UserService {
                @handler getForm
	            get /example/form (GetFormReq) returns (GetFormResp)
            }
        "#;

        let tokenizer = APITokenizer::new(source);
        for token in tokenizer {
            println!("{:#?}", token);
        }
    }
}
