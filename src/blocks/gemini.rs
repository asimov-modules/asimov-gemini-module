use asimov_sdk::flow::derive::Block;
use asimov_sdk::flow::{Block, BlockResult, BlockRuntime, InputPort, OutputPort, Port};

pub mod model;
pub use model::*;
use protoflow::PortResult;
use tracing::{debug, error, info};

/// A block that calls Gemini api.
#[derive(Block, Clone)]
pub struct Gemini {
    /// The input message stream.
    #[input]
    pub input: InputPort<Request>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<Response>,

    /// The output error message stream.
    #[output]
    pub error: OutputPort<Error>,

    #[parameter]
    pub llm_model: LlmModel,

    #[parameter]
    pub api_key: ApiKey,
}
impl Gemini {
    pub fn new(
        input: InputPort<Request>,
        output: OutputPort<Response>,
        error: OutputPort<Error>,
        llm_model: LlmModel,
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
    fn send(&self, response: &Response) -> PortResult<()> {
        info!(target:"Gemini:send", "Send Gemini result to output port");
        self.output.send(&response)
    }
    fn call(&self, input: Request) -> ResponseResult {
        info!(target: "Gemini:call", "Calling Gemini");
        call_llm(self.llm_model.clone(), self.api_key.clone(), input)
    }
    fn send_error(&self, error: &Error) -> PortResult<()> {
        info!(target:"Gemini:send_error", "Send error to the error port");
        if !self.error.is_connected() {
            info!(target:"Gemini:send_error", "Error port is not connected");
            Ok(())
        } else {
            return self.error.send(&error);
        }
    }
}

impl Block for Gemini {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult<()> {
        info!(target:"Gemini::execute", "Executing Gemini block");
        while let Some(input) = self.input.recv()? {
            if !self.output.is_connected() {
                info!(target:"Gemini::execute", "Output Port is not connected");
                continue;
            }
            // let x = self.llm_model();

            match self.call(input) {
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

    use super::model::*;
    use super::Gemini;
    use asimov_sdk::flow::{transports::MockTransport, System};

    #[test]
    fn instantiate_gemini_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Gemini::new(
                s.input(),
                s.output(),
                s.output(),
                LlmModel::Gemini1_5Flash,
                ApiKey::from_str("asdf").unwrap(),
            ));
        });
    }
}
