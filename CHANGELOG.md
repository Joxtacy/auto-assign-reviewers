# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Nothing yet

### Changed
- Nothing yet

### Fixed
- Nothing yet

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

[Unreleased]: https://github.com/Joxtacy/auto-assign-reviewers/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Joxtacy/auto-assign-reviewers/releases/tag/v0.1.0
