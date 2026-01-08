# SPDX-License-Identifier: AGPL-3.0-or-later
# rhodibot - Justfile Task Runner
#
# IMPORTANT: This file MUST be named "Justfile" (capital J) for RSR compliance.

default:
    @just --list

# Build the project
build:
    @echo "Building..."

# Run tests
test:
    @echo "Testing..."

# Run lints
lint:
    @echo "Linting..."

# Clean build artifacts
clean:
    @echo "Cleaning..."

# Format code
fmt:
    @echo "Formatting..."

# Run all checks
check: lint test

# Prepare a release
release VERSION:
    @echo "Releasing {{VERSION}}..."

