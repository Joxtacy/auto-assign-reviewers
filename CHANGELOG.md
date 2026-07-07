# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `team_slug` input to build the reviewer pool from an org team's members, combined with any manual `team_members` (requires a token with org read access)
- `exclude` input to remove specific usernames from the reviewer pool (e.g. a manager who doesn't review)

### Changed
- `team_members` is now optional when `team_slug` is set; at least one of the two is required

### Fixed
- Nothing yet

## [0.1.2] - 2026-07-07

### Added
- `number_of_reviewers` input to assign the N least-busy reviewers (default: 1)

## [0.1.1] - 2025-12-27

### Changed
- Switch to pre-built images for faster action startup

## [0.1.0] - 2025-12-25

### Added
- Initial preview release
- Intelligent workload-based reviewer assignment
- Configurable scoring weights for open PRs, lines of code, and recent reviews
- Support for teams of any size
- Automatic exclusion of PR author from assignment
- Comprehensive scoring output in action logs
- Test scripts for local development (`test-config.sh`, `test-with-real-pr.sh`)
- Full documentation (README.md, TESTING.md)
- Example workflow file

### Technical Details
- Built with Rust using octocrab for GitHub API
- Docker-based GitHub Action for easy deployment
- Support for pagination when handling many open PRs
- Rate limit handling for large repositories

[Unreleased]: https://github.com/Joxtacy/auto-assign-reviewers/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/Joxtacy/auto-assign-reviewers/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Joxtacy/auto-assign-reviewers/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Joxtacy/auto-assign-reviewers/releases/tag/v0.1.0
