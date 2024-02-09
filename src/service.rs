#![allow(dead_code)]

use indexmap::IndexMap;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{branch::alt, combinator::map, sequence::tuple};

use crate::common::{match_text_case_insensitive, match_token, IResult, Input};
use crate::token::APITokenKind::*;

#[derive(Debug, Default)]
pub struct Service {
    name: String,
    anotation: Option<IndexMap<String, String>>,
    pub handlers: Vec<Handler>,
}

#[derive(Debug)]
pub struct Handler {
    pub name: String,
    pub method: HttpMethod,
    pub path: String,
    req_type: Option<String>,
    pub resp_type: Option<String>,
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
        }
    }
}

pub fn parse_service(i: Input) -> IResult<Service> {
    tuple((
        opt(parse_service_anotation),
        delimited(
            match_token(Service),
            match_token(Identifier),
            match_token(OpenBrace),
        ),
        many0(parse_handler),
        match_token(CloseBrace),
    ))(i)
    .map(|(i, (anotation, name, handlers, _))| {
        (
            i,
            Service {
                name: name.at.to_string(),
                anotation,
                handlers,
            },
        )
    })
}

fn parse_service_anotation(i: Input) -> IResult<IndexMap<String, String>> {
    tuple((
        match_token(Server),
        delimited(
            match_token(OpenParen),
            parse_kv_pairs,
            match_token(CloseParen),
        ),
    ))(i)
    .map(|(i, (_, pairs))| (i, pairs.into_iter().collect::<IndexMap<_, _>>()))
}

fn parse_kv_pairs(i: Input) -> IResult<Vec<(String, String)>> {
    many0(tuple((
        match_token(Identifier),
        match_token(Colon),
        match_token(Identifier),
    )))(i)
    .map(|(i, pairs)| {
        (
            i,
            pairs
                .into_iter()
                .map(|(key, _, value)| (key.at.to_string(), value.at.to_string()))
                .collect::<Vec<_>>(),
        )
    })
}

fn parse_handler(i: Input) -> IResult<Handler> {
    tuple((
        match_token(Handler),
        match_token(Identifier),
        parse_http_method,
        match_token(RoutePath),
        delimited(
            opt(match_token(OpenParen)),
            opt(match_token(Identifier)),
            opt(match_token(CloseParen)),
        ),
        delimited(
            tuple((opt(match_token(RespReturns)), opt(match_token(OpenParen)))),
            opt(match_token(Identifier)),
            opt(match_token(CloseParen)),
        ),
    ))(i)
    .map(|(i, (_, name, method, path, req_type, resp_type))| {
        (
            i,
            Handler {
                name: name.at.to_string(),
                method,
                path: path.at.to_string(),
                req_type: req_type.map(|t| t.at.to_string()),
                resp_type: resp_type.map(|t| t.at.to_string()),
            },
        )
    })
}

fn parse_http_method(i: Input) -> IResult<HttpMethod> {
    alt((
        map(match_text_case_insensitive("GET"), |_| HttpMethod::GET),
        map(match_text_case_insensitive("POST"), |_| HttpMethod::POST),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn it_parse_service() {
        let source = r#"
        @server (
            group:   json
            jwt:     Auth
            timeout: 3ms
        )
        service example {
            @handler getForm
            get /example/form (GetFormReq) returns (GetFormResp)
        
            @handler postJson
            post /example/json (PostJsonReq) returns (PostJsonResp)
        }
        "#;
        let tokens = tokenize(source);
        let res = parse_service(&tokens);

        let service_res = res.unwrap().1;
        println!("{:#?}", service_res);
    }

    #[test]
    fn it_parse_service_anotation() {
        let source = r#"
            @server (
                group:   json
                jwt:     Auth
                timeout: 3m
            )
        "#;
        let tokens = tokenize(source);
        let res = parse_service_anotation(&tokens);

        let anotation_res = res.unwrap().1;
        println!("{:#?}", anotation_res);
    }

    #[test]
    fn it_parse_kv_pairs() {
        let source = r#"
            group:   json
            jwt:     Auth
            timeout: 3m
        "#;
        let tokens = tokenize(source);
        let res = parse_kv_pairs(&tokens);

        let kv_pairs_res = res.unwrap().1;
        println!("{:#?}", kv_pairs_res);
    }

    #[test]
    fn it_parse_handler() {
        let source = r#"
            @handler GetFormReq 
            get /form/req returns (GetFormReq)
        "#;
        let tokens = tokenize(source);
        let res = parse_handler(&tokens);

        let handler_res = res.unwrap().1;
        println!("{:#?}", handler_res);
    }
}
