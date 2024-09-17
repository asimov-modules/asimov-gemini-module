// This is free and unencumbered software released into the public domain.
use asimov_integrations::llm::LlmErrorTypeMessage;
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use colored::*;

use asimov_gemini_module::blocks::ApiKey;
use asimov_gemini_module::blocks::*;

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
    )
    .await;
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
            if let Some(x) = err.error {
                match x {
                    asimov_integrations::llm::llm_error_message::Error::RestError(rest_error) => {
                        print_rest_error(rest_error);
                    }
                    asimov_integrations::llm::llm_error_message::Error::ErrorType(error_type) => {
                        print_llm_error(LlmErrorTypeMessage::try_from(error_type).unwrap());
                    }
                }
            }
        }
    };
}

fn print_llm_error(err: LlmErrorTypeMessage) {
    println!(
        "{}: {}",
        "Error".bright_red().bold(),
        err.as_str_name().red().bold()
    );
}

fn print_rest_error(rest_error: asimov_integrations::error::RestErrorMessage) {
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
