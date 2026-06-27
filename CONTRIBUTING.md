# Contributing to MCP Fusion

Thank you for your interest in contributing to MCP Fusion! This document provides guidelines for contributing to the project.

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating.

## How to Contribute

### Reporting Bugs

1. Check the [existing issues](https://github.com/mcp-fusion/mcp-fusion/issues) to avoid duplicates
2. Use the **Bug Report** template when creating a new issue
3. Include detailed steps to reproduce, expected behavior, and environment info
4. Attach relevant log files from the `logs/` directory

### Suggesting Features

1. Check the [roadmap](README.md#roadmap) and existing issues
2. Use the **Feature Request** template
3. Describe the problem your feature solves and alternatives considered

### Pull Requests

1. **Fork** the repository and create your branch from `main`
2. Name your branch: `feature/description` or `fix/description`
3. Ensure your code follows the project's style guidelines
4. Run all checks before submitting:
   ```bash
   # Rust checks
   cd src-tauri
   cargo fmt --all -- --check
   cargo clippy --no-default-features -- -D warnings
   cargo test --no-default-features

   # Frontend checks
   cd ../src-frontend
   npx tsc --noEmit
   npx prettier --check "src/**/*.{ts,tsx,css,json}"
   npx vitest run
   ```
5. Write or update tests as needed
6. Update documentation for any changed behavior
7. Use the **Pull Request template**

## Development Setup

### Prerequisites

- **Node.js** >= 20
- **Rust** >= 1.77 (with `rustfmt` and `clippy`)
- **Tauri CLI** (`npm install -g @tauri-apps/cli`)

### First-Time Setup

```bash
# Clone and install
git clone https://github.com/mcp-fusion/mcp-fusion.git
cd mcp-fusion
npm install

# Run in development mode
npm run tauri:dev
```

### Project Structure

```
mcp-fusion/
├── src-tauri/              # Rust backend (Tauri)
│   ├── src/
│   │   ├── lib.rs          # Main entry, Tauri commands
│   │   ├── gateway/        # MCP transport (stdio, SSE, HTTP)
│   │   ├── orchestrator/   # Workflow scheduler
│   │   ├── storage/        # SQLite database
│   │   ├── llm.rs          # LLM intent parsing
│   │   ├── marketplace.rs  # Plugin marketplace
│   │   ├── metrics.rs      # Prometheus metrics
│   │   ├── tracing_otel.rs # OpenTelemetry tracing
│   │   └── crypto.rs       # Encryption utilities
│   └── Cargo.toml
├── src-frontend/           # React frontend
│   ├── src/
│   │   ├── pages/          # Route pages
│   │   ├── components/     # Shared components
│   │   ├── services/       # IPC service layer
│   │   └── stores/         # Zustand state
│   └── package.json
├── src-shared/             # Shared types & constants
│   ├── types/
│   └── constants.ts
└── docs/                   # Documentation
```

## Code Style

### Rust
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `anyhow::Result` for fallible operations
- Use `tracing` for logging (not `println!`)
- Prefer `map_err` over `unwrap`/`expect` in production code

### TypeScript/React
- Use functional components with hooks
- Use `useCallback` for event handlers passed as props
- Use `useMemo` for expensive computations
- Type all props and state explicitly

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add MCP resource support
fix: resolve race condition in scheduler
docs: update README with architecture diagram
refactor: extract gateway transport trait
test: add circuit breaker integration tests
```

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 License.