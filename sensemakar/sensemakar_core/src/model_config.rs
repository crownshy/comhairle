use rig::{client::CompletionClient, providers::openai};

/// Configuration for an OpenAI-compatible completion model, built at runtime
/// instead of hardcoded to a single provider/host (e.g. Bedrock exposes an
/// OpenAI-compatible endpoint, as does a local Ollama server).
#[derive(Clone, Debug)]
pub struct ModelConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

/// Builds a `rig` completion model from an OpenAI-compatible endpoint.
///
/// `rig::completion::CompletionModel` methods return `impl Future`, so the
/// trait isn't `dyn`-compatible: callers can't hold a boxed "any provider"
/// model. This builder covers the one provider shape we need (OpenAI-
/// compatible chat completions, which Bedrock and Ollama both speak) and
/// returns a concrete type generic over the model, rather than reaching for
/// a heavier enum-dispatch abstraction for providers nothing currently uses.
pub fn build_model(cfg: &ModelConfig) -> openai::completion::CompletionModel {
    let client = openai::Client::builder()
        .base_url(&cfg.base_url)
        .api_key(cfg.api_key.clone())
        .build()
        .expect("failed to build openai-compatible client")
        .completions_api();

    client.completion_model(&cfg.model)
}
