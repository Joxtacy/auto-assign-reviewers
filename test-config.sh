#!/bin/bash
# Test script to simulate GitHub Actions environment locally
# Usage: ./test-config.sh [owner/repo] [pr-number]
#
# Examples:
#   ./test-config.sh                          # Uses defaults
#   ./test-config.sh telavox/my-repo 42       # Custom repo and PR

set -e

# Check if gh CLI is available
if ! command -v gh &>/dev/null; then
  echo "❌ GitHub CLI (gh) is not installed"
  echo "Install it from: https://cli.github.com/"
  exit 1
fi

# Check if authenticated
if ! gh auth status &>/dev/null; then
  echo "❌ Not authenticated with GitHub CLI"
  echo "Run: gh auth login"
  exit 1
fi

# Parse arguments or use defaults
REPO="${1:-telavox/my-repo}"
PR_NUMBER="${2:-42}"
OWNER=$(echo "$REPO" | cut -d'/' -f1)
REPO_NAME=$(echo "$REPO" | cut -d'/' -f2)

echo "🧪 Setting up test environment..."
echo "  Repository: $REPO"
echo "  PR Number: $PR_NUMBER"

# Create a mock GitHub event file
mkdir -p /tmp/test-action
cat >/tmp/test-action/event.json <<EOF
{
  "pull_request": {
    "number": $PR_NUMBER,
    "user": {
      "login": "alice"
    }
  }
}
EOF

# Get real GitHub token from gh CLI
echo "  Getting token from gh CLI..."
GH_TOKEN=$(gh auth token)

# Set environment variables (like GitHub Actions would)
export INPUT_GITHUB_TOKEN="$GH_TOKEN"
export INPUT_TEAM_MEMBERS="alice,alice,bob"
export INPUT_WEIGHT_OPEN_PRS="10"
export INPUT_WEIGHT_LINES_PER_100="1"
export INPUT_WEIGHT_RECENT_REVIEWS="3"

# GitHub Actions automatically provides these
export GITHUB_REPOSITORY_OWNER="$OWNER"
export GITHUB_REPOSITORY="$REPO"
export GITHUB_EVENT_PATH="/tmp/test-action/event.json"

echo "✅ Environment configured"
echo ""
echo "Running the application..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ Test complete!"
