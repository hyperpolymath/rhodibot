// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration module for Rhodibot

use anyhow::Result;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// GitHub App ID
    pub app_id: Option<u64>,
    /// GitHub App private key (PEM)
    pub private_key: Option<String>,
    /// Webhook secret for signature verification
    pub webhook_secret: Option<String>,
    /// GitHub API base URL (for GitHub Enterprise)
    pub github_api_url: String,
}

impl Config {
    /// Create configuration from CLI arguments
    pub fn from_cli(cli: &crate::Cli) -> Result<Self> {
        let private_key = if let Some(ref path) = cli.private_key_path {
            Some(std::fs::read_to_string(path)?)
        } else {
            std::env::var("GITHUB_PRIVATE_KEY").ok()
        };

        Ok(Self {
            app_id: cli.app_id,
            private_key,
            webhook_secret: cli.webhook_secret.clone(),
            github_api_url: std::env::var("GITHUB_API_URL")
                .unwrap_or_else(|_| "https://api.github.com".to_string()),
        })
    }
}
