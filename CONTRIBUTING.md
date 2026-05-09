# Contributing

Thank you for your interest in farmledge-protocol.

## Getting Started

1. Fork the repository and create a branch from `main`.
2. Install Rust (stable) and the `wasm32-unknown-unknown` target:
   ```
   rustup target add wasm32-unknown-unknown
   ```
3. Run tests: `cargo test --workspace`

## Pull Requests

- Keep PRs focused — one concern per PR.
- Add or update tests for any behaviour change.
- Run `cargo clippy --workspace` and fix all warnings before opening a PR.
- Fill in the pull request template.

## Commit Messages

Use the [Conventional Commits](https://www.conventionalcommits.org/) format:
```
feat: add sesame grading validation
fix: correct storage key collision in maize-receipt
```

## Code of Conduct

Be respectful. We follow the [Contributor Covenant](https://www.contributor-covenant.org/).
