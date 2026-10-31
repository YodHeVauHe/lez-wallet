use anyhow::Context;
use sequencer_service_rpc::{RpcClient, SequencerClientBuilder};
use serde::{Deserialize, Serialize};
use url::Url;

/// Connection settings for the single sequencer selected by the user.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct EngineConfig {
    pub sequencer_url: Url,
}

impl EngineConfig {
    pub fn from_json(input: &str) -> anyhow::Result<Self> {
        let config: Self = serde_json::from_str(input).context("parse engine config")?;
        anyhow::ensure!(
            matches!(config.sequencer_url.scheme(), "http" | "https"),
            "sequencer URL must use http or https"
        );
        Ok(config)
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string_pretty(self).context("serialize engine config")
    }
}

/// Read-only facade over the upstream wallet's sequencer client implementation.
pub struct SequencerStatusClient {
    client: sequencer_service_rpc::SequencerClient,
}

impl SequencerStatusClient {
    pub fn connect(config: &EngineConfig) -> anyhow::Result<Self> {
        let client = SequencerClientBuilder::default()
            .build(config.sequencer_url.as_str())
            .context("build LEZ sequencer client")?;
        Ok(Self { client })
    }

    pub async fn last_block_id(&self) -> anyhow::Result<u64> {
        RpcClient::get_last_block_id(&self.client)
            .await
            .context("query LEZ sequencer last block ID")
    }
}

#[cfg(test)]
mod tests {
    use super::EngineConfig;

    #[test]
    fn parses_and_serializes_sequencer_url() {
        let config =
            EngineConfig::from_json(r#"{"sequencer_url":"http://127.0.0.1:3040"}"#).unwrap();
        assert_eq!(config.sequencer_url.as_str(), "http://127.0.0.1:3040/");
        assert_eq!(
            EngineConfig::from_json(&config.to_json().unwrap()).unwrap(),
            config
        );
    }

    #[test]
    fn rejects_non_http_urls() {
        assert!(EngineConfig::from_json(r#"{"sequencer_url":"file:///tmp/rpc"}"#).is_err());
    }
}
