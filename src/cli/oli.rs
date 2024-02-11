#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use super::canonicalize_path;
use super::error::TransformError;
use crate::{openapi::swagger::to_swagger, parser::parse_api, token::tokenize};

// goctl oai swagger --api <api file> -dir <output dir>
#[derive(Parser)]
#[clap(
    name = "goctl",
    version = "0.1.1",
    author = "hanxuanliang",
    about = "goctl cli[Rust Version]"
)]
pub struct Goctl {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Oai {
        #[clap(subcommand)]
        command: OaiCommand,
    },
}

#[derive(Subcommand)]
enum OaiCommand {
    Swagger {
        #[clap(long = "api", short = 'a', default_value = "example.api")]
        input_api: String,
        #[clap(long = "dir", short = 'd', default_value = ".")]
        output_dir: String,
    },
}

fn run_oli() -> Result<(), TransformError> {
    let goctl = Goctl::parse();

    match goctl.command {
        Command::Oai {
            command:
                OaiCommand::Swagger {
                    input_api,
                    output_dir,
                },
        } => {
            convert_to_swagger(&PathBuf::from(&input_api), &PathBuf::from(&output_dir))?;
            Ok(())
        }
    }
}

fn convert_to_swagger(input_api: &PathBuf, output_dir: &PathBuf) -> Result<String, TransformError> {
    let (api_path, output_dir) = (
        canonicalize_path(PathBuf::from(input_api)),
        canonicalize_path(PathBuf::from(output_dir)),
    );

    fs::create_dir_all(&output_dir).map_err(|_| TransformError::OutDirError(output_dir.clone()))?;

    let source = fs::read_to_string(&api_path).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", api_path.to_str().unwrap(), e);
        std::process::exit(1);
    });
    let (_, api_data) =
        parse_api(&(tokenize(&source))).map_err(|e| TransformError::ParseError(e.to_string()))?;

    let swagger_json = to_swagger(api_data);

    // Write to file
    let output_path = PathBuf::from(&output_dir).join("gen_swagger.json");
    let mut output_file = File::create(&output_path)?;
    writeln!(output_file, "{}", swagger_json.to_string())?;

    Ok(swagger_json.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oli() {
        let args = vec![
            "goctl",
            "oai",
            "swagger",
            "--api",
            "tests/example.api",
            "--dir",
            "tests",
        ];

        let cli = Goctl::parse_from(args);
        match cli.command {
            Command::Oai { command } => match command {
                OaiCommand::Swagger {
                    input_api,
                    output_dir,
                } => {
                    convert_to_swagger(&PathBuf::from(&input_api), &PathBuf::from(&output_dir))
                        .expect("Failed to convert to swagger");
                }
            },
        }
    }
}
