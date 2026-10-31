use anyhow::Context;
use clap::{Parser, Subcommand};
use lez_wallet_engine::{EngineConfig, SequencerStatusClient};

#[derive(Debug, Parser)]
#[command(name = "lez-wallet", version, about = "LEZ wallet command line")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Status {
        #[arg(long, default_value = "https://testnet.lez.logos.co")]
        sequencer_url: url::Url,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Show {
        #[arg(long, default_value = "https://testnet.lez.logos.co")]
        sequencer_url: url::Url,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Config {
            command: ConfigCommand::Show { sequencer_url },
        } => {
            let config = EngineConfig { sequencer_url };
            println!("{}", config.to_json()?);
        }
        Command::Status { sequencer_url } => {
            let config = EngineConfig { sequencer_url };
            let client = SequencerStatusClient::connect(&config)?;
            let block_id = client
                .last_block_id()
                .await
                .context("read sequencer status")?;
            println!("Last block ID: {block_id}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Cli;

    #[test]
    fn parses_version_flag() {
        assert!(Cli::try_parse_from(["lez-wallet", "--version"]).is_err());
    }

    #[test]
    fn parses_config_show_and_status() {
        assert!(Cli::try_parse_from(["lez-wallet", "config", "show"]).is_ok());
        assert!(Cli::try_parse_from(["lez-wallet", "status"]).is_ok());
        assert!(
            Cli::try_parse_from([
                "lez-wallet",
                "status",
                "--sequencer-url",
                "http://localhost:3040"
            ])
            .is_ok()
        );
    }
}
