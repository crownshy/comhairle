use rig_core::DynModel;
use rig_core::operation::Completion;
use rig_core::providers::ollama::OllamaConfig;
use sensemakar::model_config::{self, ModelConfig};

/// Which completion backend a `ModelSelection` talks to. Mirrors
/// `sensemakar_cli`'s `Provider` enum so the eval CLI's model flags behave
/// the same way the main CLI's do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    /// A local (or remote) Ollama server - the default for the test suite.
    Ollama,
    /// Any OpenAI-shaped chat-completions endpoint authenticated with a
    /// static bearer key.
    OpenAiCompatible,
    /// AWS Bedrock, authenticated through the ambient AWS credential chain.
    Bedrock,
}

impl Provider {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "ollama" => Some(Self::Ollama),
            "openai-compatible" | "openai_compatible" => Some(Self::OpenAiCompatible),
            "bedrock" => Some(Self::Bedrock),
            _ => None,
        }
    }
}

/// Everything needed to build a completion model, regardless of provider.
/// `base_url`/`api_key`/`region` are interpreted according to `provider` -
/// e.g. `region` is ignored unless `provider` is `Bedrock`.
pub struct ModelSelection {
    pub provider: Provider,
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub region: Option<String>,
}

impl ModelSelection {
    pub fn build(&self) -> DynModel<Completion> {
        match self.provider {
            Provider::Ollama => {
                let mut config = OllamaConfig::new().with_base_url(&self.base_url);
                if !self.api_key.is_empty() {
                    config = config.with_api_key(self.api_key.clone());
                }
                config.client().completion(&self.model).erase()
            }
            Provider::OpenAiCompatible => model_config::build_model(&ModelConfig::OpenAiCompatible {
                base_url: self.base_url.clone(),
                api_key: self.api_key.clone(),
                model: self.model.clone(),
            }),
            Provider::Bedrock => model_config::build_model(&ModelConfig::Bedrock {
                model: self.model.clone(),
                region: self.region.clone(),
            }),
        }
    }
}
