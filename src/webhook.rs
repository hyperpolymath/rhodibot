// SPDX-License-Identifier: AGPL-3.0-or-later

//! Webhook handling module

use anyhow::Result;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use tracing::{info, warn};

use crate::config::Config;
use crate::github::{CreateCheckRun, CheckRunOutput, GitHubClient};
use crate::rsr;

type HmacSha256 = Hmac<Sha256>;

/// Verify GitHub webhook signature
pub fn verify_signature(secret: &str, payload: &str, signature: &str) -> bool {
    let signature = signature.strip_prefix("sha256=").unwrap_or(signature);

    let Ok(signature_bytes) = hex::decode(signature) else {
        return false;
    };

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(payload.as_bytes());

    mac.verify_slice(&signature_bytes).is_ok()
}

/// Handle push event
pub async fn handle_push(config: &Config, body: &str) -> Result<()> {
    let event: PushEvent = serde_json::from_str(body)?;

    info!(
        "Push to {}/{} on branch {}",
        event.repository.owner.login, event.repository.name, event.r#ref
    );

    // Only check on default branch pushes
    let default_branch = format!("refs/heads/{}", event.repository.default_branch);
    if event.r#ref != default_branch {
        info!("Skipping non-default branch push");
        return Ok(());
    }

    // Run compliance check
    let client = GitHubClient::new(config);
    let report = rsr::check_compliance(
        config,
        &event.repository.owner.login,
        &event.repository.name,
    )
    .await?;

    // Create check run
    let conclusion = if report.percentage >= 70.0 {
        "success"
    } else if report.percentage >= 50.0 {
        "neutral"
    } else {
        "failure"
    };

    let check_run = CreateCheckRun {
        name: "RSR Compliance".to_string(),
        head_sha: event.after,
        status: "completed".to_string(),
        conclusion: Some(conclusion.to_string()),
        output: Some(CheckRunOutput {
            title: format!("RSR Score: {:.0}%", report.percentage),
            summary: report.summary.clone(),
            text: Some(format_report_text(&report)),
        }),
    };

    client
        .create_check_run(
            &event.repository.owner.login,
            &event.repository.name,
            &check_run,
        )
        .await?;

    info!("Created check run for push");

    Ok(())
}

/// Handle pull request event
pub async fn handle_pull_request(config: &Config, body: &str) -> Result<()> {
    let event: PullRequestEvent = serde_json::from_str(body)?;

    info!(
        "Pull request #{} {} on {}/{}",
        event.pull_request.number,
        event.action,
        event.repository.owner.login,
        event.repository.name
    );

    // Only check on opened/synchronized
    if event.action != "opened" && event.action != "synchronize" {
        return Ok(());
    }

    // Run compliance check
    let client = GitHubClient::new(config);
    let report = rsr::check_compliance(
        config,
        &event.repository.owner.login,
        &event.repository.name,
    )
    .await?;

    // Create check run
    let conclusion = if report.percentage >= 70.0 {
        "success"
    } else if report.percentage >= 50.0 {
        "neutral"
    } else {
        "failure"
    };

    let check_run = CreateCheckRun {
        name: "RSR Compliance".to_string(),
        head_sha: event.pull_request.head.sha,
        status: "completed".to_string(),
        conclusion: Some(conclusion.to_string()),
        output: Some(CheckRunOutput {
            title: format!("RSR Score: {:.0}%", report.percentage),
            summary: report.summary.clone(),
            text: Some(format_report_text(&report)),
        }),
    };

    client
        .create_check_run(
            &event.repository.owner.login,
            &event.repository.name,
            &check_run,
        )
        .await?;

    Ok(())
}

/// Handle repository event
pub async fn handle_repository(config: &Config, body: &str) -> Result<()> {
    let event: RepositoryEvent = serde_json::from_str(body)?;

    info!(
        "Repository {} {}/{}",
        event.action, event.repository.owner.login, event.repository.name
    );

    // On repository creation, create an issue with RSR checklist
    if event.action == "created" {
        let client = GitHubClient::new(config);

        let issue_body = r#"## RSR Compliance Checklist

Welcome to the hyperpolymath organization! Please ensure your repository follows the Rhodium Standard Repository guidelines.

### Required Files
- [ ] `README.adoc` - Project documentation (AsciiDoc format)
- [ ] `LICENSE.txt` - License file (AGPL-3.0, MIT, Apache-2.0, or MPL-2.0)
- [ ] `SECURITY.md` - Security policy
- [ ] `CONTRIBUTING.md` - Contribution guidelines
- [ ] `CODE_OF_CONDUCT.md` - Code of conduct
- [ ] `.claude/CLAUDE.md` - AI assistant instructions
- [ ] `STATE.scm` - Project state (Guile Scheme)
- [ ] `META.scm` - Meta information (Guile Scheme)
- [ ] `ECOSYSTEM.scm` - Ecosystem position (Guile Scheme)

### Language Policy (CCCP)
**Allowed:** ReScript, Rust, Deno, Gleam, Bash, Julia, Ada, OCaml
**Banned:** TypeScript, Node.js, npm, Go, Python (except SaltStack)

### CI/CD
- [ ] GitHub Actions workflows in `.github/workflows/`
- [ ] SHA-pinned actions
- [ ] `permissions: read-all` on workflows

For more details, see the [RSR Documentation](https://github.com/hyperpolymath/rhodium-standard-repositories).

---
*This issue was created automatically by Rhodibot*
"#;

        match client
            .create_issue(
                &event.repository.owner.login,
                &event.repository.name,
                "[Rhodibot] RSR Compliance Checklist",
                issue_body,
                &["documentation", "rsr-compliance"],
            )
            .await
        {
            Ok(issue) => info!("Created RSR checklist issue: {}", issue.html_url),
            Err(e) => warn!("Failed to create issue: {}", e),
        }
    }

    Ok(())
}

/// Handle installation event
pub async fn handle_installation(_config: &Config, body: &str) -> Result<()> {
    let event: InstallationEvent = serde_json::from_str(body)?;

    info!(
        "Installation {} for {}",
        event.action,
        event.installation.account.login
    );

    Ok(())
}

/// Format report as markdown text
fn format_report_text(report: &rsr::ComplianceReport) -> String {
    let mut text = String::new();

    text.push_str("## Detailed Results\n\n");

    let categories = [
        ("Documentation", rsr::CheckCategory::Documentation),
        ("Security", rsr::CheckCategory::Security),
        ("Governance", rsr::CheckCategory::Governance),
        ("Structure", rsr::CheckCategory::Structure),
        ("Language Policy", rsr::CheckCategory::LanguagePolicy),
    ];

    for (cat_name, cat) in categories {
        let cat_checks: Vec<_> = report
            .checks
            .iter()
            .filter(|c| matches!((&c.category, &cat),
                (rsr::CheckCategory::Documentation, rsr::CheckCategory::Documentation) |
                (rsr::CheckCategory::Security, rsr::CheckCategory::Security) |
                (rsr::CheckCategory::Governance, rsr::CheckCategory::Governance) |
                (rsr::CheckCategory::Structure, rsr::CheckCategory::Structure) |
                (rsr::CheckCategory::LanguagePolicy, rsr::CheckCategory::LanguagePolicy)
            ))
            .collect();

        if cat_checks.is_empty() {
            continue;
        }

        text.push_str(&format!("### {}\n\n", cat_name));

        for check in cat_checks {
            let icon = match check.status {
                rsr::CheckStatus::Pass => ":white_check_mark:",
                rsr::CheckStatus::Fail => ":x:",
                rsr::CheckStatus::Warn => ":warning:",
                rsr::CheckStatus::Skip => ":fast_forward:",
            };

            text.push_str(&format!(
                "- {} **{}**: {} ({}/{})\n",
                icon, check.name, check.message, check.points, check.max_points
            ));
        }

        text.push('\n');
    }

    text
}

// Event types

#[derive(Debug, Deserialize)]
struct PushEvent {
    r#ref: String,
    after: String,
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct PullRequestEvent {
    action: String,
    pull_request: PullRequest,
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    number: u64,
    head: PullRequestHead,
}

#[derive(Debug, Deserialize)]
struct PullRequestHead {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryEvent {
    action: String,
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct InstallationEvent {
    action: String,
    installation: Installation,
}

#[derive(Debug, Deserialize)]
struct Installation {
    account: Account,
}

#[derive(Debug, Deserialize)]
struct Account {
    login: String,
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
    default_branch: String,
    owner: Owner,
}

#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
}
