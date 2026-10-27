pub mod config;

use sensemakar::model_config;

use self::config::{SensemakarModelConfig, SensemakarModelProvider};

/// Builds the `sensemakar` crate's model config from ours, resolving the fields
/// each provider actually needs. Shared by workers that call the sensemaker model.
pub fn build_model_config(
    model_cfg: &SensemakarModelConfig,
) -> Result<model_config::ModelConfig, String> {
    Ok(match model_cfg.provider {
        SensemakarModelProvider::OpenAiCompatible => model_config::ModelConfig::OpenAiCompatible {
            base_url: model_cfg.base_url.clone().ok_or(
                "sensemaker_model.base_url is required for the openai_compatible provider",
            )?,
            api_key: model_cfg
                .api_key
                .clone()
                .ok_or("sensemaker_model.api_key is required for the openai_compatible provider")?,
            model: model_cfg.model.clone(),
        },
        SensemakarModelProvider::Bedrock => model_config::ModelConfig::Bedrock {
            model: model_cfg.model.clone(),
            region: model_cfg.region.clone(),
        },
    })
}
