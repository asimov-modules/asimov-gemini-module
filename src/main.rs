// This is free and unencumbered software released into the public domain.

use std::str::FromStr;

use clap::{Parser, ValueEnum};
use colored::*;

use asimov_gemini_module::blocks::*;
use asimov_gemini_module::blocks::{ApiKey};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let request = LlmRequestMessage {
        prompt: cli.prompt,
        max_tokens: cli.max_tokens,
    };

    let gemini_model = cli.model.into();

    let result = call_gemini(
        gemini_model,
        ApiKey::from_str(&cli.api_key).unwrap(),
        request,
    ).await;
    print_response(result);

    Ok(())
}

#[derive(Parser)]
#[command(name = "LLM Requester")]
#[command(about = "CLI tool to interact with various Gemini models", long_about = None)]
struct Cli {
    #[arg(short, long)]
    prompt: String,

    #[arg(short = 't', long)]
    max_tokens: Option<u32>,

    #[arg(short = 'k', long = "api-key")]
    api_key: String,

    #[arg(short = 'm', long)]
    model: ModelType,
}

#[derive(ValueEnum, Clone)]
enum ModelType {
    Flash,
    Pro,
}

impl From<ModelType> for GeminiModel {
    fn from(model: ModelType) -> Self {
        match model {
            ModelType::Flash => GeminiModel::Gemini1_5Flash,
            ModelType::Pro => GeminiModel::Gemini1_5Pro,
        }
    }
}

fn print_response(result: ResponseResult) {
    match result {
        Ok(response) => {
            println!(
                "{}: {}",
                "Model".bright_blue().bold(),
                response.model().as_str_name().green()
            );
            println!(
                "{}: {}",
                "Sub Model".bright_blue().bold(),
                response.sub_model().as_str_name().green()
            );

            println!("{}", "Response:".bright_blue().bold());
            println!("{}", response.response);
        }
        Err(err) => {
            println!(
                "{}: {}",
                "Error".bright_red().bold(),
                convert_to_string(&err.error_type, || err.error_type().as_str_name()).red().bold()
            );

            if let Some(rest_error) = err.rest_error {
                println!(
                    "{}: {}",
                    "Rest Error Type".bright_red().bold(),
                    rest_error.error_type().as_str_name().red().bold()
                );

                if let Some(message) = &rest_error.message {
                    println!(
                        "{}: {}",
                        "Rest Error Message".bright_red().bold(),
                        message.red().bold()
                    );
                }
            }
        }
    };
}

fn convert_to_string<F>(enum_value: &Option<i32>, value_function: F) -> &'static str
    where
        F: Fn() -> &'static str,
{
    match enum_value {
        None => "",
        Some(_) => value_function(),
    }
}

