#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{openapi::swagger::to_swagger, parser::parse_api, token::tokenize};

use super::canonicalize_path;

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

fn oli() {
    let goctl = Goctl::parse();
    match goctl.command {
        Command::Oai { command } => match command {
            OaiCommand::Swagger {
                input_api,
                output_dir,
            } => {
                let api_path = canonicalize_path(PathBuf::from(input_api));
                let output_dir = canonicalize_path(PathBuf::from(output_dir));

                fs::create_dir_all(&output_dir).expect("Failed to create output directory");

                let source = fs::read_to_string(&api_path).unwrap_or_else(|e| {
                    eprintln!("Error reading file '{}': {}", api_path.to_str().unwrap(), e);
                    std::process::exit(1);
                });

                let input = tokenize(&source);
                let result = parse_api(&input);

                let api_data = result.unwrap().1;
                let swagger_json = to_swagger(api_data);
                let mut output_file =
                    File::create(output_dir).expect("Failed to create output file");
                writeln!(output_file, "{}", swagger_json.to_string())
                    .expect("Failed to write Generate Swagger JSON");
            }
        },
    }
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
                    let api_path = canonicalize_path(PathBuf::from(input_api));
                    let output_dir = canonicalize_path(PathBuf::from(output_dir));

                    let source = fs::read_to_string(&api_path).unwrap_or_else(|e| {
                        eprintln!("Error reading file '{}': {}", api_path.to_str().unwrap(), e);
                        std::process::exit(1);
                    });

                    let input = tokenize(&source);
                    let result = parse_api(&input);

                    let api_data = result.unwrap().1;
                    let swagger_json = to_swagger(api_data);
                    let mut output_file = File::create(output_dir.join("gen_swagger.json"))
                        .expect("Failed to create output file");
                    writeln!(output_file, "{}", swagger_json.to_string())
                        .expect("Failed to write Swagger JSON");
                }
            },
        }
    }
}
