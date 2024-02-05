#![allow(dead_code)]

use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{branch::alt, combinator::map};

use crate::common::{match_text, match_token, IResult, Input};
use crate::token::APITokenKind::*;

#[derive(Debug)]
pub struct StructDef {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    name: String,
    field_type: FieldType,
    tag: Option<String>,
}

#[derive(Debug, PartialEq)]
enum FieldType {
    Int,
    Int32,
    Int64,
    String,
    Bool,
    Array(Box<FieldType>),
}

fn parse_struct(i: Input) -> IResult<StructDef> {
    tuple((
        match_token(Type),
        match_token(Identifier),
        match_token(Struct),
        match_token(OpenBrace),
        many0(parse_field),
        match_token(CloseBrace),
    ))(i)
    .map(|(i, (_, name, _, _, fields, _))| {
        (
            i,
            StructDef {
                name: name.at.to_string(),
                fields,
            },
        )
    })
}

fn parse_field(i: Input) -> IResult<Field> {
    tuple((
        match_token(Identifier),
        parse_field_type,
        opt(match_token(TagAnnotation)),
    ))(i)
    .map(|(i, (name, data_type, tag_token))| {
        (
            i,
            Field {
                name: name.at.to_string(),
                field_type: data_type,
                tag: tag_token.map(|t| t.at.to_string()),
            },
        )
    })
}

// parse_field_type parses a field type.
fn parse_field_type(i: Input) -> IResult<FieldType> {
    alt((
        map(match_text("string"), |_| FieldType::String),
        map(match_text("int"), |_| FieldType::Int),
        map(match_text("int32"), |_| FieldType::Int32),
        map(match_text("int64"), |_| FieldType::Int64),
        map(match_text("bool"), |_| FieldType::Bool),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token::tokenize;

    #[test]
    fn test_parse_struct() {
        let source = r#"
            type GetFormReq struct {
                Name  string `form:"name,omitempty"`
                Age   int64  `form:"age" json:"age"`
            }
        "#;
        let input = tokenize(source);
        let result = parse_struct(&input);

        let struct_def = result.unwrap().1;
        println!("{:#?}", struct_def);
    }

    #[test]
    fn test_parse_field() {
        let source = r#"name string `json:"name"`"#;
        // let source = r#"name string"#;
        let input = tokenize(source);
        let result = parse_field(&input);

        let field_var = result.unwrap().1;
        println!("{:#?}", field_var);
    }

    #[test]
    fn test_parse_field_type() {
        let source = "string";
        let input = tokenize(source);
        let result = parse_field_type(&input);

        let field_type = result.unwrap().1;
        assert_eq!(field_type, FieldType::String);
    }
}
