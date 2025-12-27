// SPDX-License-Identifier: AGPL-3.0-or-later

//! RSR (Rhodium Standard Repository) compliance checking module

use anyhow::Result;
use serde::Serialize;

use crate::config::Config;
use crate::github::GitHubClient;

/// RSR Compliance Report
#[derive(Debug, Serialize)]
pub struct ComplianceReport {
    pub owner: String,
    pub repo: String,
    pub score: u8,
    pub max_score: u8,
    pub percentage: f32,
    pub checks: Vec<Check>,
    pub summary: String,
}

/// Individual compliance check
#[derive(Debug, Serialize)]
pub struct Check {
    pub name: String,
    pub category: CheckCategory,
    pub status: CheckStatus,
    pub points: u8,
    pub max_points: u8,
    pub message: String,
}

/// Check categories
#[derive(Debug, Serialize, Clone, Copy)]
pub enum CheckCategory {
    #[serde(rename = "documentation")]
    Documentation,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "governance")]
    Governance,
    #[serde(rename = "structure")]
    Structure,
    #[serde(rename = "language_policy")]
    LanguagePolicy,
}

/// Check status
#[derive(Debug, Serialize, Clone, Copy)]
pub enum CheckStatus {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "skip")]
    Skip,
}

/// Required files for RSR compliance
const REQUIRED_FILES: &[(&str, &str, CheckCategory, u8)] = &[
    ("README.adoc", "AsciiDoc README", CheckCategory::Documentation, 5),
    ("LICENSE.txt", "License file", CheckCategory::Governance, 5),
    ("SECURITY.md", "Security policy", CheckCategory::Security, 5),
    ("CONTRIBUTING.md", "Contributing guidelines", CheckCategory::Documentation, 3),
    ("CODE_OF_CONDUCT.md", "Code of conduct", CheckCategory::Governance, 3),
    (".claude/CLAUDE.md", "AI assistant instructions", CheckCategory::Structure, 2),
    ("STATE.scm", "Project state file", CheckCategory::Structure, 3),
    ("META.scm", "Meta information", CheckCategory::Structure, 3),
    ("ECOSYSTEM.scm", "Ecosystem position", CheckCategory::Structure, 3),
];

/// Banned file patterns (anti-patterns)
const BANNED_PATTERNS: &[(&str, &str, CheckCategory)] = &[
    ("package-lock.json", "npm lock file (use Deno)", CheckCategory::LanguagePolicy),
    ("yarn.lock", "Yarn lock file (use Deno)", CheckCategory::LanguagePolicy),
    ("pnpm-lock.yaml", "pnpm lock file (use Deno)", CheckCategory::LanguagePolicy),
    ("bun.lockb", "Bun lock file (use Deno)", CheckCategory::LanguagePolicy),
    ("go.mod", "Go module (use Rust)", CheckCategory::LanguagePolicy),
    ("go.sum", "Go checksum (use Rust)", CheckCategory::LanguagePolicy),
];

/// Check repository compliance with RSR
pub async fn check_compliance(config: &Config, owner: &str, repo: &str) -> Result<ComplianceReport> {
    let client = GitHubClient::new(config);
    let mut checks = Vec::new();
    let mut total_score = 0u8;
    let mut max_score = 0u8;

    // Check required files
    for (file, description, category, points) in REQUIRED_FILES {
        max_score += points;
        let exists = client.file_exists(owner, repo, file).await;

        if exists {
            total_score += points;
            checks.push(Check {
                name: file.to_string(),
                category: *category,
                status: CheckStatus::Pass,
                points: *points,
                max_points: *points,
                message: format!("{} found", description),
            });
        } else {
            checks.push(Check {
                name: file.to_string(),
                category: *category,
                status: CheckStatus::Fail,
                points: 0,
                max_points: *points,
                message: format!("{} missing", description),
            });
        }
    }

    // Check for banned patterns
    for (file, description, category) in BANNED_PATTERNS {
        let exists = client.file_exists(owner, repo, file).await;

        if exists {
            checks.push(Check {
                name: format!("no-{}", file),
                category: *category,
                status: CheckStatus::Fail,
                points: 0,
                max_points: 0,
                message: format!("{} detected - RSR violation", description),
            });
        }
    }

    // Check for workflow files
    max_score += 5;
    let has_workflows = client.file_exists(owner, repo, ".github/workflows").await;
    if has_workflows {
        total_score += 5;
        checks.push(Check {
            name: ".github/workflows".to_string(),
            category: CheckCategory::Structure,
            status: CheckStatus::Pass,
            points: 5,
            max_points: 5,
            message: "GitHub Actions workflows found".to_string(),
        });
    } else {
        checks.push(Check {
            name: ".github/workflows".to_string(),
            category: CheckCategory::Structure,
            status: CheckStatus::Fail,
            points: 0,
            max_points: 5,
            message: "No GitHub Actions workflows".to_string(),
        });
    }

    // Check license type
    if let Ok(repo_info) = client.get_repository(owner, repo).await {
        max_score += 5;
        if let Some(license) = repo_info.license {
            let approved_licenses = ["agpl-3.0", "apache-2.0", "mit", "mpl-2.0", "lgpl-3.0"];
            if approved_licenses.contains(&license.key.as_str()) {
                total_score += 5;
                checks.push(Check {
                    name: "license-type".to_string(),
                    category: CheckCategory::Governance,
                    status: CheckStatus::Pass,
                    points: 5,
                    max_points: 5,
                    message: format!("Approved license: {}", license.name),
                });
            } else {
                checks.push(Check {
                    name: "license-type".to_string(),
                    category: CheckCategory::Governance,
                    status: CheckStatus::Warn,
                    points: 2,
                    max_points: 5,
                    message: format!("Non-standard license: {}", license.name),
                });
                total_score += 2;
            }
        } else {
            checks.push(Check {
                name: "license-type".to_string(),
                category: CheckCategory::Governance,
                status: CheckStatus::Fail,
                points: 0,
                max_points: 5,
                message: "No license detected".to_string(),
            });
        }
    }

    let percentage = if max_score > 0 {
        (total_score as f32 / max_score as f32) * 100.0
    } else {
        0.0
    };

    let summary = if percentage >= 90.0 {
        "Excellent RSR compliance".to_string()
    } else if percentage >= 70.0 {
        "Good RSR compliance with minor issues".to_string()
    } else if percentage >= 50.0 {
        "Partial RSR compliance - improvements needed".to_string()
    } else {
        "Poor RSR compliance - significant work required".to_string()
    };

    Ok(ComplianceReport {
        owner: owner.to_string(),
        repo: repo.to_string(),
        score: total_score,
        max_score,
        percentage,
        checks,
        summary,
    })
}
