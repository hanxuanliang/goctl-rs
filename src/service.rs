#![allow(dead_code)]

use nom::combinator::opt;
use nom::multi::many0;
use nom::{branch::alt, combinator::map, sequence::tuple};

use crate::common::{match_text_case_insensitive, match_token, IResult, Input};
use crate::token::APITokenKind::*;

#[derive(Debug)]
struct Service {
    name: String,
    handlers: Vec<Handler>,
}

#[derive(Debug)]
struct Handler {
    name: String,
    method: HttpMethod,
    path: String,
    req_type: Option<String>,
    resp_type: Option<String>,
}

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
}

fn parse_service(i: Input) -> IResult<Service> {
    tuple((
        match_token(Service),
        match_token(Identifier),
        match_token(OpenBrace),
        many0(parse_handler),
        match_token(CloseBrace),
    ))(i)
    .map(|(i, (_, name, _, handlers, _))| {
        (
            i,
            Service {
                name: name.at.to_string(),
                handlers,
            },
        )
    })
}

fn parse_handler(i: Input) -> IResult<Handler> {
    tuple((
        match_token(Handler),
        match_token(Identifier),
        parse_http_method,
        match_token(RoutePath),
        opt(match_token(OpenParen)),
        opt(match_token(Identifier)),
        opt(match_token(CloseParen)),
        opt(match_token(RespReturns)),
        opt(match_token(OpenParen)),
        opt(match_token(Identifier)),
        opt(match_token(CloseParen)),
    ))(i)
    .map(
        |(i, (_, name, method, path, _, req_type, _, _, _, resp_type, _))| {
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
        },
    )
}

fn parse_http_method(i: Input) -> IResult<HttpMethod> {
    alt((
        map(match_text_case_insensitive("GET"), |_| HttpMethod::Get),
        map(match_text_case_insensitive("POST"), |_| HttpMethod::Post),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn it_parse_service() {
        let source = r#"
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
