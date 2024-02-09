#![allow(dead_code)]

use std::vec;

use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{branch::alt, combinator::map};
use serde::{Deserialize, Serialize};

use crate::common::{match_text, match_token, IResult, Input};
use crate::token::APITokenKind::*;

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    tag: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    Int,
    Int32,
    Int64,
    String,
    Bool,
    Array(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
}

// parse_struct_stmt parses a struct statement.
pub fn parse_struct_stmt(i: Input) -> IResult<Vec<StructDef>> {
    alt((parse_nest_struct, parse_many_struct))(i)
}

pub fn parse_struct_stmt1(input: Input) -> IResult<Vec<StructDef>> {
    let mut structs = Vec::new();
    let mut i = input;

    while !i.is_empty() {
        match alt((parse_nest_struct, parse_struct_to_vec))(i) {
            Ok((next_input, mut parsed_structs)) => {
                structs.append(&mut parsed_structs); // Append parsed structs to the collection
                i = next_input; // Update the input to the remaining unparsed part
            }
            _ => break, // If parsing fails, exit the loop
        }
    }

    Ok((i, structs))
}

// parse_many_struct parses many struct statements.
/// type GetFormReq struct {
///     Name  string `form:"name,omitempty"`
///     Age   int64  `form:"age" json:"age"`
/// }
/// type GetFormResp struct {
///     Total int64 `json:"total"`
/// }
fn parse_many_struct(i: Input) -> IResult<Vec<StructDef>> {
    many0(parse_one_struct)(i)
}

// parse_nest_struct parses a nested struct statement.
/// type (
///     GetFormReq struct {
///         Name  string `form:"name,omitempty"`
///         Age   int64  `form:"age" json:"age"`
///     }
///     GetFormResp struct {
///         Total int64 `json:"total"`
///     }
/// )
fn parse_nest_struct(i: Input) -> IResult<Vec<StructDef>> {
    delimited(
        tuple((match_token(Type), match_token(OpenParen))),
        parse_many_struct,
        match_token(CloseParen),
    )(i)
}

fn parse_struct_to_vec(i: Input) -> IResult<Vec<StructDef>> {
    map(parse_one_struct, |s| vec![s])(i)
}

// parse_one_struct parses a single struct statement.
fn parse_one_struct(i: Input) -> IResult<StructDef> {
    tuple((
        delimited(
            opt(match_token(Type)),
            match_token(Identifier),
            opt(match_token(Struct)),
        ),
        match_token(OpenBrace),
        many0(parse_field),
        match_token(CloseBrace),
    ))(i)
    .map(|(i, (name, _, fields, _))| {
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
        parse_array_type,
        parse_map_type,
        map(match_text("string"), |_| FieldType::String),
        map(match_text("int"), |_| FieldType::Int),
        map(match_text("int32"), |_| FieldType::Int32),
        map(match_text("int64"), |_| FieldType::Int64),
        map(match_text("bool"), |_| FieldType::Bool),
    ))(i)
}

fn parse_array_type(i: Input) -> IResult<FieldType> {
    tuple((
        match_token(OpenBracket),
        match_token(CloseBracket),
        parse_field_type,
    ))(i)
    .map(|(i, (_, _, data_type))| (i, FieldType::Array(Box::new(data_type))))
}

// parse_map_type parses a map type: map[keyType]valueType.
fn parse_map_type(i: Input) -> IResult<FieldType> {
    tuple((
        match_token(MapDataType),
        match_token(OpenBracket),
        parse_field_type,
        match_token(CloseBracket),
        parse_field_type,
    ))(i)
    .map(|(i, (_, _, key_type, _, value_type))| {
        (i, FieldType::Map(Box::new(key_type), Box::new(value_type)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token::tokenize;

    #[test]
    fn test_parse_struct() {
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
        
        "#;
        let input = tokenize(source);
        let result = parse_struct_stmt1(&input);

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

    #[test]
    fn test_parse_field_type_array() {
        let source = "[]int";
        let input = tokenize(source);
        let result = parse_array_type(&input);

        let field_type = result.unwrap().1;
        println!("{:#?}", field_type);
    }

    #[test]
    fn test_parse_field_type_map() {
        let source = "map[string]map[string][]int";
        let input = tokenize(source);
        let result = parse_map_type(&input);

        let field_type = result.unwrap().1;
        println!("{:#?}", field_type);
    }
}
