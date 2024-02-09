#![allow(dead_code)]

use nom::sequence::tuple;

use crate::{
    common::{IResult, Input},
    service::{parse_service, Service},
    struct_ref::{parse_struct_stmt1, StructDef},
};

#[derive(Debug)]
pub struct APIStmt {
    pub type_struct: Vec<StructDef>,
    pub service: Service,
}

pub fn parse_api(i: Input) -> IResult<APIStmt> {
    tuple((parse_struct_stmt1, parse_service))(i).map(|(i, (type_struct, service))| {
        (
            i,
            APIStmt {
                type_struct,
                service,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::tokenize;

    #[test]
    fn test_parse_api() {
        let source = r#"
        type (
            PostFormReq struct {
                Name    string   `form:"name"`
                Age     int      `form:"age"`
            }
            PostFormResp struct {
                Total int64 `json:"total"`
            }
        )
        type GetFormReq struct {
            Name  string `form:"name,omitempty"`
            Age   int64  `form:"age" json:"age"`
        }
        type GetFormResp struct {
            Total int64 `json:"total"`
        }

        @server (
            group:   json
            jwt:     Auth
            timeout: 3m
        )
        service example {
            @handler getForm
            get /example/form (GetFormReq) returns (GetFormResp)
        
            @handler postJson
            post /example/json (PostJsonReq) returns (PostJsonResp)
        }
        "#;
        let input = tokenize(source);

        let result = parse_api(&input);
        let api_var = result.unwrap().1;
        println!("{:#?}", api_var);
    }
}
