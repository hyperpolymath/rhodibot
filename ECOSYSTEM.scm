;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Project ecosystem position for rhodibot

(ecosystem
 (version "1.0")
 (name "rhodibot")
 (type "compliance-bot")
 (purpose "RSR structural compliance validator. The foundational bot in the
   gitbot-fleet — all other bots depend on rhodibot completing first because
   structural compliance is a prerequisite for all other analysis.")

 (position-in-ecosystem
   (role "verifier-tier-bot")
   (layer "structural-compliance")
   (fleet-tier "verifier")
   (execution-order 1)
   (description "First bot to execute in the fleet pipeline. Validates that
     repos follow RSR (Rhodium Standard Repository) conventions. Its findings
     inform all downstream bots — glambot needs structure for presentation
     analysis, seambot needs layout for boundary detection, finishbot needs
     compliance for release readiness."))

 (related-projects
   (parent
     (gitbot-fleet
       (relationship "fleet-member")
       (description "Rhodibot is one of six specialized bots and the most
         foundational — other bots depend on its structural findings.")
       (integration "Publishes structural compliance findings via shared-context API")))
   (engine
     (hypatia
       (relationship "rules-engine")
       (description "Hypatia provides RSR rules as Logtalk predicates.
         Rhodibot executes these rules against repo structure.")
       (integration "Receives RSR rule configurations from hypatia")))
   (standard
     (rsr-template-repo
       (relationship "reference-implementation")
       (description "RSR template defines the gold standard that rhodibot validates
         against. All 17 required workflows, directory structure, and file
         conventions come from this template.")
       (repo "hyperpolymath/rsr-template-repo")))
   (dependents
     (glambot
       (relationship "depends-on-rhodibot")
       (description "Glambot needs rhodibot's structural findings to know
         where to look for presentation issues."))
     (seambot
       (relationship "depends-on-rhodibot")
       (description "Seambot needs rhodibot's directory layout analysis to
         identify architectural boundaries."))
     (finishingbot
       (relationship "depends-on-rhodibot")
       (description "Finishbot needs rhodibot's compliance results as part
         of release readiness validation.")))
   (infrastructure
     (git-private-farm
       (relationship "propagation")
       (description "RSR compliance must be consistent across all forge mirrors."))
     (robot-repo-automaton
       (relationship "fix-executor")
       (description "When rhodibot identifies fixable compliance issues
         (missing files, wrong SPDX headers), robot-repo-automaton applies fixes."))))

 (what-this-is
   "The foundational compliance bot for the entire gitbot-fleet"
   "An RSR (Rhodium Standard Repository) structure validator"
   "A language policy enforcer (bans TypeScript, Python, Go, npm)"
   "A Verifier-tier bot that all other bots depend on"
   "The first bot to run in every fleet pipeline execution")

 (what-this-is-not
   "Not a code quality checker — that is glambot's responsibility"
   "Not a formal verifier — that is echidnabot"
   "Not a release gater — that is finishbot (rhodibot feeds it findings)"
   "Not an architecture auditor — that is seambot"
   "Not a standalone linter — it integrates with the gitbot-fleet"))
