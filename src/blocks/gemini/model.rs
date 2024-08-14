pub use asimov_integrations::ApiKey;
pub use asimov_integrations::llm::gemini::call_gemini as call_llm;
pub use asimov_integrations::llm::gemini::GeminiModel as LlmModel;
pub use asimov_integrations::llm::proto::LlmError as Error;
pub use asimov_integrations::llm::proto::LlmRequest as Request;
pub use asimov_integrations::llm::proto::LlmResponse as Response;

pub type ResponseResult = Result<Response, Error>;
