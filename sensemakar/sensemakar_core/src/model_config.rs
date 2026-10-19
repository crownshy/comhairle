use rig_bedrock::client::BedrockRuntime;
use rig_core::DynModel;
use rig_core::operation::Completion;
use rig_core::providers::openai::OpenAIConfig;

/// Configuration for the completion model the describers run against, built
/// at runtime instead of hardcoded to a single provider.
#[derive(Clone, Debug)]
pub enum ModelConfig {
    /// Any OpenAI-shaped chat-completions endpoint (Ollama, a literal OpenAI
    /// key, or a gateway that speaks the same wire), authenticated with a
    /// static key.
    OpenAiCompatible {
        base_url: String,
        api_key: String,
        model: String,
    },
    /// AWS Bedrock, authenticated through the ambient AWS credential chain
    /// (env vars, shared config, instance metadata/IRSA) rather than a
    /// static key, so an IAM role attached to the running task/instance is
    /// all production needs.
    Bedrock {
        model: String,
        /// Falls back to the credential chain's region when unset.
        region: Option<String>,
    },
}

/// Builds a model handle erased to its completion operation, so callers hold
/// one type regardless of which provider `cfg` selects.
pub fn build_model(cfg: &ModelConfig) -> DynModel<Completion> {
    match cfg {
        ModelConfig::OpenAiCompatible {
            base_url,
            api_key,
            model,
        } => OpenAIConfig::new(api_key.clone())
            .with_base_url(base_url.clone())
            .client()
            .chat(model)
            .erase(),
        ModelConfig::Bedrock { model, region } => {
            let mut builder = BedrockRuntime::builder();
            if let Some(region) = region {
                builder = builder.region(region.clone());
            }
            builder.build().completion(model).erase()
        }
    }
}
