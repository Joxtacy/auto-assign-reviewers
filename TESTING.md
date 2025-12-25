# Testing with act (Local GitHub Actions)

`act` is a tool that runs GitHub Actions locally using Docker. This is the most realistic way to test your action before pushing to GitHub.

## Important: Environment Variable Naming

For Docker container actions, input names should use **underscores** (not dashes) in `action.yml`:
- Input: `github_token` → Env var: `INPUT_GITHUB_TOKEN`
- Input: `team_members` → Env var: `INPUT_TEAM_MEMBERS`

This is because bash cannot handle dashes in environment variable names, and it's standard practice for Docker-based GitHub Actions.

## Installation

### macOS
```bash
brew install act
```

### Linux
```bash
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

### Other platforms
See: https://github.com/nektos/act

## Usage

### 1. Create a test workflow in your project repo

Create `.github/workflows/test-action.yml`:

```yaml
name: Test Auto Assign (Local)

on:
  pull_request:
    types: [opened, ready_for_review]

jobs:
  assign:
    runs-on: ubuntu-latest
    steps:
      - name: Assign Reviewer
        uses: ./  # Uses local action directory
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          team-members: 'alice,bob,charlie'
```

### 2. Run the workflow locally

```bash
# Run all workflows
act pull_request

# Run specific workflow
act pull_request -W .github/workflows/test-action.yml

# Run with a GitHub token (for real API calls)
act pull_request -s GITHUB_TOKEN=$(gh auth token)

# Verbose output for debugging
act pull_request -v
```

### 3. Simulate a PR event

Create `.github/workflows/act-event.json`:

```json
{
  "pull_request": {
    "number": 123,
    "user": {
      "login": "alice"
    },
    "head": {
      "ref": "feature-branch"
    },
    "base": {
      "ref": "main"
    }
  }
}
```

Then run:
```bash
act pull_request -e .github/workflows/act-event.json
```

## Benefits of using act

✅ Tests the full Docker build process
✅ Validates action.yml configuration
✅ Simulates the exact GitHub Actions environment
✅ Can test with real GitHub API calls
✅ Faster feedback loop than pushing to GitHub

## Tips

- Use `-n` flag for dry run (doesn't execute, just shows what would run)
- Use `--container-architecture linux/amd64` on M1/M2 Macs if you have issues
- First run will be slow (Docker build), subsequent runs are cached

## Common Issues

**Docker image too large:**
- Act downloads large runner images by default
- Use `act -P ubuntu-latest=catthehacker/ubuntu:act-latest` for smaller images

**Permission errors:**
- Run act with proper Docker permissions
- May need to add your user to docker group

**Network issues:**
- Act runs in Docker, ensure Docker has network access
- Use `--network host` if needed
