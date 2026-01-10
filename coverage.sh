#!/bin/bash

# rcal Coverage Measurement Script
# Generates coverage reports for the rcal project

set -e

echo "Running tests and generating coverage report..."

# Generate HTML coverage report
cargo llvm-cov --html --output-dir coverage-report

# Generate summary
echo "Coverage Summary:"
cargo llvm-cov --summary-only

echo "HTML report saved to: coverage-report/html/index.html"

# Check against targets
echo ""
echo "Coverage Targets:"
echo "- Core logic (persistence.rs, date_utils.rs, daemon.rs): 80% minimum"
echo "- TUI code (main.rs, ui.rs): 60% minimum"
echo ""
echo "Run 'cargo llvm-cov --summary-only' to check current coverage."