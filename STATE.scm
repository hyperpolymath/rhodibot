;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Current project state for rhodibot

(define state
  '((metadata
     (version "0.2.0")
     (schema-version "1.0")
     (created "2026-01-15")
     (updated "2026-02-05")
     (project "rhodibot")
     (repo "hyperpolymath/rhodibot"))

    (project-context
     (name "rhodibot")
     (tagline "RSR structural compliance validator — ensures repos follow Rhodium Standard Repository conventions")
     (tech-stack ("Rust 1.83+" "Clap CLI" "GitHub App integration"
                  "Check Runs API" "Webhook handlers")))

    (current-position
     (phase "active-development")
     (overall-completion 50)
     (components
       (("Required Files Checker" . ((status . "complete") (completion . 100)
          (description . "Validates presence of README.adoc, LICENSE, SECURITY.md,
            CODE_OF_CONDUCT.md, CONTRIBUTING.md, .claude/CLAUDE.md")))
        ("SCM File Validator" . ((status . "complete") (completion . 100)
          (description . "Validates STATE.scm, META.scm, ECOSYSTEM.scm structure")))
        ("Directory Layout Checker" . ((status . "complete") (completion . 100)
          (description . "Validates RSR directory structure conventions")))
        ("Language Policy Enforcer" . ((status . "complete") (completion . 100)
          (description . "Detects banned languages (TypeScript, Python, Go, npm).
            Enforces Rust, ReScript, Deno, Gleam, Elixir preference.")))
        ("Banned Patterns (CCCP)" . ((status . "complete") (completion . 100)
          (description . "Enforces CCCP policy: no npm, yarn, bun, pnpm")))
        ("GitHub App Integration" . ((status . "partial") (completion . 40)
          (description . "Webhook handlers and check runs on PRs")))
        ("Auto-Issue Creation" . ((status . "partial") (completion . 30)
          (description . "Creates RSR compliance checklist issues")))
        ("CII Badge Automation" . ((status . "planned") (completion . 0)
          (description . "OpenSSF Best Practices badge registration")))
        ("Fleet Integration" . ((status . "planned") (completion . 0)
          (description . "Integration with gitbot-fleet shared-context")))))
     (working-features
       ("RSR required files presence validation"
        "SCM file structure validation (STATE, META, ECOSYSTEM)"
        "Directory layout compliance checking"
        "Language policy enforcement (detect banned languages)"
        "CCCP pattern enforcement (no npm/yarn/bun/pnpm)"
        "CLI interface for local validation")))

    (route-to-mvp
     (milestones
       ((name "Core Validation")
        (status "complete")
        (items ("Required files checker" "SCM file validator"
                "Directory layout checker" "Language policy enforcer"
                "CCCP banned patterns")))
       ((name "Forge Integration")
        (status "in-progress")
        (items ("GitHub App setup" "Webhook handlers" "Check Runs API"
                "Auto-issue creation" "CII badge automation")))
       ((name "Fleet & Release")
        (status "planned")
        (items ("Fleet integration" "Integration tests" "Documentation"
                "v1.0 release")))))

    (blockers-and-issues
     (critical ())
     (high ("GitHub App integration incomplete — webhook handling partial"))
     (medium ("No fleet integration yet" "CII badge automation not started"))
     (low ()))

    (critical-next-actions
     (immediate ("Complete GitHub App webhook handlers"
                 "Add Check Runs support for PR validation"))
     (this-week ("Implement auto-issue creation for compliance checklists"
                 "Add fleet integration with shared-context"))
     (this-month ("CII badge automation"
                  "Run rhodibot validation across all 500+ repos"
                  "v1.0 release")))))
