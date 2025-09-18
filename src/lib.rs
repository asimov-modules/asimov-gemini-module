// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

use asimov_module::{
    prelude::*,
    secrecy::{ExposeSecret, SecretString},
    tracing,
};
use core::error::Error;
use serde_json::{Value, json};

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    #[builder(default = "https://generativelanguage.googleapis.com")]
    pub endpoint: String,

    #[builder(default = "gemini-2.5-flash")]
    pub model: String,

    pub max_tokens: Option<usize>,

    #[builder(into)]
    pub api_key: SecretString,
}

pub fn generate(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    let mut req = json!({
        "contents": {
            "parts": [
                {"text": input.as_ref()}
            ]
        },
    });

    if let Some(max_tokens) = options.max_tokens {
        req["generationConfig"] = json!({"maxOutputTokens": max_tokens})
    }

    let mut resp = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("asimov-gemini-module")
        .build()
        .new_agent()
        .post(format!(
            "{}/v1beta/models/{}:generateContent",
            options.endpoint, options.model
        ))
        .header("x-goog-api-key", options.api_key.expose_secret())
        .header("content-type", "application/json")
        .send_json(&req)
        .inspect_err(|e| tracing::error!("HTTP request failed: {e}"))?;
    tracing::debug!(response = ?resp);

    let status = resp.status();
    tracing::debug!(status = status.to_string());

    let resp: Value = resp
        .body_mut()
        .read_json()
        .inspect_err(|e| tracing::error!("unable to read HTTP response body: {e}"))?;
    tracing::debug!(body = ?resp);

    if !status.is_success() {
        tracing::error!("Received an error response: {status}");

        // {
        //   "error": {
        //     "code": 400,
        //     "message": "API key not valid. Please pass a valid API key.",
        //     "status": "INVALID_ARGUMENT"
        //   }
        // }
        if let Some(message) = resp["error"]["message"].as_str() {
            return Err(message.into());
        }
    }

    // {
    //   "candidates": [
    //     {
    //       "content": {
    //         "parts": [
    //           {
    //             "text": "..."
    //           }
    //         ],
    //         "role": "model"
    //       },
    //       "finishReason": "STOP",
    //       "index": 0
    //     }
    //   ],
    //   "usageMetadata": {
    //     "promptTokenCount": 8,
    //     "candidatesTokenCount": 15,
    //     "totalTokenCount": 1191,
    //     "promptTokensDetails": [
    //       {
    //         "modality": "TEXT",
    //         "tokenCount": 8
    //       }
    //     ],
    //     "thoughtsTokenCount": 1168
    //   },
    //   "modelVersion": "gemini-2.5-flash",
    //   "responseId": "..."
    // }

    let mut responses = Vec::new();

    if let Some(chunks) = resp["candidates"].as_array() {
        for chunk in chunks {
            if let Some(content) = chunk["content"].as_object() {
                if content["role"].as_str().is_none_or(|r| r != "model") {
                    continue;
                }

                content["parts"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|p| p["text"].as_str())
                    .for_each(|t| responses.push(t.to_string()));
            };

            if let Some(stop_reason) = chunk["finishReason"].as_str() {
                tracing::debug!(stop_reason);
            }
        }
    }

    Ok(responses)
}
