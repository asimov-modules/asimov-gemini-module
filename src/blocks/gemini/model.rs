pub use asimov_integrations::llm::gemini::call_gemini as call_llm;
pub use asimov_integrations::llm::model::proto::LlmError as Error;
pub use asimov_integrations::llm::model::proto::LlmRequest as Request;
pub use asimov_integrations::llm::model::proto::LlmResponse as Response;

pub use asimov_integrations::llm::gemini::GeminiModel as LlmModel;

pub use asimov_integrations::ApiKey;
pub type ResponseResult = Result<Response, Error>;
