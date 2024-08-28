use asimov_sdk::flow::{Block, BlockResult, BlockRuntime, InputPort, OutputPort, Port};
use asimov_sdk::flow::derive::Block;
use asimov_sdk::flow::PortResult;
use tokio::runtime;
use tokio::runtime::Runtime;
use tracing::{debug, error, info};

pub use model::*;

mod model;

/// A block that calls Gemini api.
#[derive(Block, Clone)]
pub struct Gemini {
    /// The input message stream.
    #[input]
    pub input: InputPort<LlmRequestMessage>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<LlmResponseMessage>,

    /// The output error message stream.
    #[output]
    pub error: OutputPort<LlmErrorMessage>,

    #[parameter]
    pub llm_model: GeminiModel,

    #[parameter]
    pub api_key: ApiKey,
}

impl Gemini {
    pub fn new(
        input: InputPort<LlmRequestMessage>,
        output: OutputPort<LlmResponseMessage>,
        error: OutputPort<LlmErrorMessage>,
        llm_model: GeminiModel,
        api_key: ApiKey,
    ) -> Self {
        Self {
            input,
            output,
            error,
            llm_model,
            api_key,
        }
    }
    fn send(&self, response: &LlmResponseMessage) -> PortResult<()> {
        info!(target:"Gemini:send", "Send Gemini result to output port");
        self.output.send(response)
    }
    fn call(&self, input: LlmRequestMessage, rt: &Runtime) -> ResponseResult {
        info!(target: "Gemini:call", "Calling Gemini");
        let result = rt.block_on(async {
            call_gemini(self.llm_model.clone(), self.api_key.clone(), input).await
        });
        result
    }
    fn send_error(&self, error: &LlmErrorMessage) -> PortResult<()> {
        info!(target:"Gemini:send_error", "Send error to the error port");
        if !self.error.is_connected() {
            info!(target:"Gemini:send_error", "Error port is not connected");
            Ok(())
        } else {
            return self.error.send(error);
        }
    }
}

impl Block for Gemini {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult<()> {
        info!(target:"Gemini::execute", "Executing Gemini block");
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        while let Some(input) = self.input.recv()? {
            if !self.output.is_connected() {
                info!(target:"Gemini::execute", "Output Port is not connected");
                continue;
            }

            match self.call(input, &rt) {
                Ok(response) => {
                    debug!(target:"Gemini:execute",?response, "Gemini response");
                    self.send(&response)?;
                }
                Err(err) => {
                    error!(target: "Gemini:execute","Error calling Gemini");
                    self.send_error(&(err.into()))?;
                }
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use asimov_sdk::flow::{System, transports::MockTransport};

    use super::Gemini;
    use super::model::*;

    #[test]
    fn instantiate_gemini_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Gemini::new(
                s.input(),
                s.output(),
                s.output(),
                GeminiModel::Gemini1_5Flash,
                ApiKey::from_str("asdf").unwrap(),
            ));
        });
    }
}
