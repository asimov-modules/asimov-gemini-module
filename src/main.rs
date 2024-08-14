// This is free and unencumbered software released into the public domain.

use std::str::FromStr;

use clap::{Parser, ValueEnum};
use colored::*;

use asimov_gemini_module::blocks::*;
use asimov_gemini_module::blocks::{ApiKey, LlmModel};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let request = Request {
        prompt: cli.prompt,
        max_tokens: cli.max_tokens,
    };

    let gemini_model = cli.model.into();

    let result = call_llm(
        gemini_model,
        ApiKey::from_str(&cli.api_key).unwrap(),
        request,
    );
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

impl From<ModelType> for LlmModel {
    fn from(model: ModelType) -> Self {
        match model {
            ModelType::Flash => LlmModel::Gemini1_5Flash,
            ModelType::Pro => LlmModel::Gemini1_5Pro,
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

            if response.response.to_lowercase().contains("error") {
                println!("{}", "An error was detected in the response.".red().bold());
            }
        }
        Err(err) => {
            println!(
                "{}: {}",
                "Error".bright_red().bold(),
                err.error().as_str_name().red().bold()
            );

            if let Some(integration_error) = err.integration_error.as_ref() {
                println!(
                    "{}: {}",
                    "Integration Error Type".bright_red().bold(),
                    integration_error.error_type().as_str_name().red().bold()
                );

                if let Some(message) = &integration_error.message {
                    println!(
                        "{}: {}",
                        "Integration Error Message".bright_red().bold(),
                        message.red().bold()
                    );
                }
            }
        }
    };
}
