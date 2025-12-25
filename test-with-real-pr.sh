#!/bin/bash
# Test with a real PR from GitHub
# Usage: ./test-with-real-pr.sh owner/repo pr-number
#
# Example:
#   ./test-with-real-pr.sh telavox/my-repo 123

set -e

if [ $# -lt 2 ]; then
	echo "Usage: $0 owner/repo pr-number"
	echo "Example: $0 telavox/my-repo 123"
	exit 1
fi

REPO="$1"
PR_NUMBER="$2"
OWNER=$(echo "$REPO" | cut -d'/' -f1)
REPO_NAME=$(echo "$REPO" | cut -d'/' -f2)

echo "🔍 Fetching real PR data from GitHub..."

# Check if gh CLI is available
if ! command -v gh &>/dev/null; then
	echo "❌ GitHub CLI (gh) is not installed"
	exit 1
fi

# Fetch the actual PR data
echo "  Fetching PR #$PR_NUMBER from $REPO..."
PR_DATA=$(gh pr view "$PR_NUMBER" --repo "$REPO" --json number,author)

if [ -z "$PR_DATA" ]; then
	echo "❌ Failed to fetch PR data. Does it exist?"
	exit 1
fi

# Extract author
PR_AUTHOR=$(echo "$PR_DATA" | jq -r '.author.login')

echo "✅ Found PR #$PR_NUMBER by @$PR_AUTHOR"

# Create event file with real data
mkdir -p /tmp/test-action
echo "$PR_DATA" | jq '{pull_request: {number: .number, user: {login: .author.login}}}' >/tmp/test-action/event.json

# Get real GitHub token
GH_TOKEN=$(gh auth token)

# Set environment variables
export INPUT_GITHUB_TOKEN="$GH_TOKEN"
export INPUT_TEAM_MEMBERS="alice,bob,charlie" # Update with your team
export INPUT_WEIGHT_OPEN_PRS="10"
export INPUT_WEIGHT_LINES_PER_100="1"
export INPUT_WEIGHT_RECENT_REVIEWS="3"
export GITHUB_REPOSITORY_OWNER="$OWNER"
export GITHUB_REPOSITORY="$REPO"
export GITHUB_EVENT_PATH="/tmp/test-action/event.json"

echo ""
echo "Running the application with real PR data..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ Test complete!"
