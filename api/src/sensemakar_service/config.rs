use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SensemakarModelProvider {
    /// Any OpenAI-shaped chat-completions endpoint (Ollama, a literal OpenAI
    /// key, or a gateway that speaks the same wire), authenticated with a
    /// static `api_key`.
    #[default]
    OpenAiCompatible,
    /// AWS Bedrock, authenticated through the ambient AWS credential chain
    /// (an IAM role in production) rather than a static key.
    Bedrock,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SensemakarModelConfig {
    #[serde(default)]
    pub provider: SensemakarModelProvider,
    /// Required for `provider = openai_compatible`; unused for `bedrock`.
    pub base_url: Option<String>,
    /// Required for `provider = openai_compatible`; unused for `bedrock`,
    /// which authenticates through the AWS credential chain instead.
    pub api_key: Option<String>,
    pub model: String,
    /// `provider = bedrock` only. Falls back to the credential chain's
    /// region when unset.
    pub region: Option<String>,
}
