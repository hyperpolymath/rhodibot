;; SPDX-License-Identifier: PMPL-1.0-or-later
;; META.scm - Meta-level project information for rhodibot

(define meta
  '((architecture-decisions
     ((adr-001
       (status "accepted")
       (date "2026-01-15")
       (context "Need a bot that validates RSR compliance across 500+ repos")
       (decision "Build rhodibot in Rust with Clap CLI and GitHub App integration.
         Validate against rsr-template-repo conventions.")
       (consequences "Consistent compliance checking. Rust ensures performance
         for scanning large repos. GitHub App enables PR-level validation."))

      (adr-002
       (status "accepted")
       (date "2026-01-20")
       (context "Language policy must be enforced automatically")
       (decision "Detect banned languages (TypeScript, Python, Go) and package
         managers (npm, yarn, bun, pnpm) by file extension and config file presence.
         Report as errors that block release.")
       (consequences "Prevents accidental introduction of banned languages.
         Enforces the migration path from banned to allowed languages."))

      (adr-003
       (status "accepted")
       (date "2026-01-25")
       (context "Other bots depend on structural compliance data")
       (decision "Rhodibot is Verifier tier with execution-order 1. Always runs
         first. Publishes findings to shared-context for downstream bots.")
       (consequences "Clean dependency chain. Downstream bots can assume structural
         data is available. Execution order is deterministic."))

      (adr-004
       (status "proposed")
       (date "2026-02-05")
       (context "Need to validate all 17 required workflows from rsr-template-repo")
       (decision "Add workflow validation: check that all 17 standard workflows
         are present with correct SHA pins and permissions.")
       (consequences "Catches workflow drift. Ensures security best practices.
         May need periodic updates as workflow SHAs change."))))

    (development-practices
     (code-style "Rust with rustfmt. Clap for CLI argument parsing.")
     (security "GitHub App JWT authentication. No shell injection. cargo-audit.")
     (testing "Unit tests for all validators. Integration tests against template repo.")
     (versioning "Semantic versioning")
     (documentation "README.adoc. STATE.scm for project tracking.")
     (branching "Main branch protected. Feature branches. PRs required."))

    (design-rationale
     ((why-first-in-pipeline
       "Structural compliance is the foundation. You cannot check presentation
        quality (glambot), architectural boundaries (seambot), or release
        readiness (finishbot) if the basic structure is wrong. Rhodibot ensures
        the prerequisites that all other bots depend on.")
      (why-language-enforcement
       "The language policy exists to prevent fragmentation across 500+ repos.
        Manual enforcement does not scale. Automated detection at PR time catches
        violations before they merge.")
      (why-rsr
       "RSR (Rhodium Standard Repository) conventions enable fleet automation.
        Every bot knows where to find files, what structure to expect, and what
        conventions to enforce. Without RSR, each bot would need per-repo configuration.")))))
