#![allow(dead_code)]

use nom::{branch::alt, combinator::map, sequence::pair};

use crate::common::{match_text, match_token, IResult, Input};
use crate::token::APITokenKind::*;

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

fn parse_field(i: Input) -> IResult<Field> {
    map(
        pair(match_token(Identifier), parse_field_type),
        |(name, data_type)| Field {
            name: name.at.to_string(),
            field_type: data_type,
            tag: None,
        },
    )(i)
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
    use crate::token::tokenize;

    use super::*;

    #[test]
    fn test_parse_field() {
        let source = r#"name string `json:"name"`"#;

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
