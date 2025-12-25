# Auto Assign PR Reviewer

**Automatically assign reviewers to pull requests based on current workload.**

Stop playing reviewer roulette! This GitHub Action intelligently assigns PR reviewers by analyzing:
- 📊 How many PRs each person is currently reviewing
- 📏 Lines of code in those reviews
- 🕐 Recent review activity (last 7 days)

The person with the **lowest workload score** gets assigned automatically.

---

## Quick Start

### 1. Add the workflow to your repository

Create `.github/workflows/auto-assign-reviewer.yml`:

```yaml
name: Auto Assign Reviewer

on:
  pull_request:
    types: [opened, ready_for_review]

jobs:
  assign:
    runs-on: ubuntu-latest
    # Only run if no reviewers are already assigned
    if: github.event.pull_request.requested_reviewers[0] == null
    
    steps:
      - name: Assign Reviewer
        uses: Joxtacy/auto-assign-reviewers@v0.1.0
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          team_members: 'alice,bob,charlie'  # Your team's GitHub usernames
```

### 2. Open a PR

That's it! The action will automatically assign the least busy reviewer.

---

## How It Works

### Scoring System

For each team member (excluding the PR author), the action calculates a workload score:

```
score = (open_prs × 10) + (lines_in_review ÷ 100 × 1) + (recent_reviews × 3)
```

**Lower score = less busy = gets assigned!**

### Example

| Reviewer | Open PRs | Lines in Review | Recent Reviews | Score | Assigned? |
|----------|----------|-----------------|----------------|-------|-----------|
| Alice    | 1        | 300             | 2              | 19    | ✅         |
| Bob      | 2        | 500             | 3              | 34    |           |
| Charlie  | 3        | 800             | 1              | 41    |           |

Alice has the lowest score (19), so she gets assigned!

---

## Configuration

### Basic Configuration

```yaml
- uses: Joxtacy/auto-assign-reviewers@v0.1.0
  with:
    github_token: ${{ secrets.GITHUB_TOKEN }}
    team_members: 'alice,bob,charlie'
```

### Advanced Configuration

Customize the scoring weights to match your team's preferences:

```yaml
- uses: Joxtacy/auto-assign-reviewers@v0.1.0
  with:
    github_token: ${{ secrets.GITHUB_TOKEN }}
    team_members: 'alice,bob,charlie'
    
    # Adjust these weights to prioritize different factors
    weight_open_prs: 15          # Default: 10
    weight_lines_per_100: 2      # Default: 1
    weight_recent_reviews: 5     # Default: 3
```

### Configuration Options

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `github_token` | GitHub token (use `secrets.GITHUB_TOKEN`) | Yes | - |
| `team_members` | Comma-separated GitHub usernames | Yes | - |
| `weight_open_prs` | Weight for currently open PRs | No | `10` |
| `weight_lines_per_100` | Weight per 100 lines of code | No | `1` |
| `weight_recent_reviews` | Weight for reviews in last 7 days | No | `3` |

---

## Tips & Best Practices

### Skip Auto-Assignment When Needed

You can still manually assign reviewers - the action only runs when no reviewers are already assigned:

```yaml
if: github.event.pull_request.requested_reviewers[0] == null
```

### Adjust Weights for Your Team

- **Heavy reviewers?** Increase `weight_open_prs` to prioritize people with fewer open PRs
- **Large PRs common?** Increase `weight_lines_per_100` to account for PR size
- **Fast reviews?** Lower `weight_recent_reviews` since people clear their queue quickly

### Multiple Teams

Different repos can use different team lists and weights. Just adjust the workflow file in each repository.

---

## Troubleshooting

### Action runs but doesn't assign anyone

- Verify all usernames in `team_members` have repository access
- Check that at least one team member isn't the PR author
- Ensure `GITHUB_TOKEN` has write permissions to PRs

### Incorrect reviewer assigned

- Check the action logs to see the calculated scores
- Adjust weights if the scoring doesn't match your team's needs
- Verify team members list includes everyone who should review

### GitHub API rate limits

The action makes several API calls per run. If you have many open PRs (50+), you might hit rate limits. Consider:
- Running the action only on specific branches
- Using GitHub's personal access token with higher rate limits

---

## Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes this action better for everyone.

### Getting Started

1. **Fork the repository**: Click the "Fork" button at the top of this page
2. **Clone your fork**: 
   ```bash
   git clone https://github.com/YOUR-USERNAME/auto-assign-reviewers.git
   cd auto-assign-reviewers
   ```
3. **Create a branch**: 
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Development Setup

This project uses Rust. You'll need:
- [Rust toolchain](https://rustup.rs/) (1.92+)
- [Docker](https://www.docker.com/) (for testing the full action)

**Local testing:**
```bash
# Run config parsing tests
./test-config.sh

# Test with a real PR (requires gh CLI)
./test-with-real-pr.sh owner/repo pr-number
```

**Testing the full action:**

See [TESTING.md](TESTING.md) for detailed instructions on using `act` to test the complete GitHub Action locally.

### Making Changes

1. **Make your changes** in a feature branch
2. **Test thoroughly** using the test scripts
3. **Update documentation** if you're changing behavior
4. **Commit with clear messages**:
   ```bash
   git commit -m "Add: Support for team-based weighting"
   ```

### Submitting Changes

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
2. **Open a Pull Request** from your fork to this repository
3. **Describe your changes** in the PR description:
   - What problem does it solve?
   - How did you test it?
   - Any breaking changes?

### Code Guidelines

- **Rust code**: Follow standard Rust conventions (`cargo fmt`, `cargo clippy`)
- **Error handling**: Use `anyhow::Result` with context
- **Documentation**: Add doc comments for public functions
- **Testing**: Include test scripts or examples when applicable

### Areas We'd Love Help With

- 🐛 **Bug fixes**: Found an issue? Fix it and send a PR!
- 📚 **Documentation**: Improve examples, clarify instructions
- ✨ **Features**: 
  - Support for multiple teams within one repo
  - Timezone-aware assignment
  - Reviewer expertise matching
  - Slack/Discord notifications
- 🧪 **Testing**: Add unit tests, integration tests
- 🌍 **Localization**: Translate README to other languages

### Questions?

- Open an [issue](https://github.com/Joxtacy/auto-assign-reviewers/issues) for bugs or feature requests
- Start a [discussion](https://github.com/Joxtacy/auto-assign-reviewers/discussions) for questions or ideas

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

## Acknowledgments

Built with:
- [octocrab](https://github.com/XAMPPRocky/octocrab) - GitHub API client for Rust
- [tokio](https://tokio.rs/) - Async runtime
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling

---

**Made with ❤️ for teams tired of manual reviewer assignment**
