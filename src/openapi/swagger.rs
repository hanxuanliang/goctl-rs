#![allow(dead_code)]

use serde::Serialize;
use serde_json::{json, Value};

use crate::{parser::APIStmt, struct_ref::FieldType};

#[derive(Default)]
pub struct Swagger {
    paths: Value,
    definitions: Value,
}

#[derive(Serialize)]
struct SwaggerField {
    #[serde(flatten)]
    field_type: FieldType,
    description: String,
}

impl Swagger {
    fn new() -> Swagger {
        Swagger {
            paths: json!({}),
            definitions: json!({}),
        }
    }

    fn append_path(&mut self, path: &str, method: &str, operation: Value) {
        let path_entry = self.paths.as_object_mut().unwrap();
        if !path_entry.contains_key(path) {
            path_entry.insert(path.to_string(), json!({}));
        }

        let method_entry = path_entry.get_mut(path).unwrap().as_object_mut().unwrap();
        method_entry.insert(method.to_string(), operation);
    }

    fn append_def(&mut self, name: &str, def: Value) {
        let def_entry = self.definitions.as_object_mut().unwrap();
        def_entry.insert(name.to_string(), def);
    }

    fn to_json(&self) -> Value {
        json!({
            "openapi": "3.0.3",
            "info": {
                "version": "1.0.0",
                "title": "Generated Swagger API"
            },
            "paths": self.paths,
            "components": {
                "schemas": self.definitions,
            },
        })
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self.to_json()).unwrap()
    }
}

pub fn to_swagger(api_data: APIStmt) -> Swagger {
    let mut swagger = Swagger::new();

    for struct_def in api_data.type_struct {
        let struct_name = struct_def.name;
        let props = struct_def
            .fields
            .iter()
            .map(|field| {
                (
                    field.name.to_lowercase(),
                    serde_json::to_value(&SwaggerField {
                        field_type: field.field_type.clone(),
                        description: field.name.to_string(),
                    })
                    .unwrap_or(json!({})),
                )
            })
            .collect::<serde_json::Map<_, _>>();

        swagger.append_def(
            &struct_name,
            json!({
                "type": "object",
                "properties": props,
            }),
        );
    }

    for handler in api_data.service.handlers {
        let operation = json!({
            "summary": handler.name,
            "responses": {
                "200": match &handler.resp_type {
                    Some(resp_type) => {
                        json!({
                            "description": "successful operation",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": format!("#/components/schemas/{}", resp_type),
                                    },
                                },
                            },
                        })
                    }
                    None => json!({"description": "OK"}),
                },
            },
        });

        swagger.append_path(&handler.path, &handler.method.to_string(), operation)
    }

    swagger
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_api;
    use crate::token::tokenize;

    use super::*;

    #[test]
    // cargo test --package goctl-rs --lib -- openapi::swagger::tests::it_to_swagger --exact --nocapture
    fn it_to_swagger() {
        let source = r#"
        type (
            PostFormReq struct {
                Name    string   `form:"name"`
                Age     []int      `form:"age"`
            }
            PostFormResp struct {
                Total int64 `json:"total"`
                State []string `json:"state"`
            }
        )
        type GetFormReq struct {
            Name  string `form:"name,omitempty"`
            Age   int64  `form:"age" json:"age"`
        }
        type GetFormResp struct {
            Total int64 `json:"total"`
            Times []int32 `json:"times"`
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
            post /example/json (PostFormReq) returns (PostFormResp)
        }
        "#;
        let input = tokenize(source);
        let result = parse_api(&input);

        let api_data = result.unwrap().1;
        let swagger = to_swagger(api_data);
        println!("{}", swagger.to_string());
    }
}
