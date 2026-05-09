# Contributing to SoroForge CLI

Thank you for your interest in contributing! SoroForge is an async-first open-source project — all coordination happens through GitHub Issues and Pull Requests.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Repository Structure](#repository-structure)
4. [Development Workflow](#development-workflow)
5. [Coding Standards](#coding-standards)
6. [Submitting a Pull Request](#submitting-a-pull-request)
7. [Issue Labels](#issue-labels)

---

## Code of Conduct

This project follows the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Be respectful and constructive.

---

## Getting Started

### Prerequisites

| Tool | Minimum Version |
|---|---|
| Rust | stable (1.77+) |
| Node.js | 20 LTS |
| npm | 10+ |
| Docker | 24+ (for sandbox tests) |
| GitHub CLI (`gh`) | 2.x |

### Setup

```bash
git clone https://github.com/Stellar-Project-Hub/SoroForge-CLI.git
cd SoroForge-CLI

# Rust workspace
cargo build

# Node.js workspaces
npm install
```

---

## Repository Structure

```
SoroForge-CLI/
├── cli/                        # Binary crate — soroforge executable
│   └── src/
│       ├── main.rs             # Clap command tree & async runtime setup
│       └── commands/           # One module per subcommand
│           ├── init.rs         # soroforge init
│           ├── sandbox.rs      # soroforge sandbox
│           ├── deploy.rs       # soroforge deploy
│           └── watch.rs        # soroforge watch
│
├── crates/                     # Internal Rust library crates
│   ├── soroforge-core/         # Shared types: SoroForgeConfig, SoroForgeError
│   ├── soroforge-sandbox/      # Docker-based local node lifecycle
│   └── soroforge-deploy/       # WASM upload + contract instantiation
│
├── packages/                   # TypeScript/Node.js packages (npm workspaces)
│   ├── sdk/                    # @soroforge/sdk — typed TS client
│   └── hot-reload/             # @soroforge/hot-reload — chokidar watcher
│
├── tests/
│   └── integration/            # End-to-end tests (require Docker)
│
├── docs/                       # Architecture decision records (ADRs) & guides
├── scripts/                    # Utility shell scripts
├── .github/workflows/ci.yml    # CI: Rust lint/test + Node lint/test
├── Cargo.toml                  # Cargo workspace manifest
├── package.json                # npm workspace manifest
└── soroforge.example.toml      # Annotated example project config
```

### Where to make changes

| What you want to change | Where to look |
|---|---|
| Add/modify a CLI flag or subcommand | `cli/src/commands/<command>.rs` |
| Change shared config or error types | `crates/soroforge-core/src/` |
| Sandbox Docker logic | `crates/soroforge-sandbox/src/sandbox.rs` |
| Deploy pipeline | `crates/soroforge-deploy/src/deployer.rs` |
| TypeScript SDK | `packages/sdk/src/` |
| Hot-reload watcher | `packages/hot-reload/src/watcher.ts` |
| CI pipeline | `.github/workflows/ci.yml` |

---

## Development Workflow

1. **Pick an issue** — look for `good first issue` or `help wanted` labels.
2. **Comment on the issue** to signal you're working on it.
3. **Create a branch** from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```
4. **Make your changes** and ensure all checks pass locally (see below).
5. **Open a Pull Request** against `main`.

### Local checks

```bash
# Rust
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all

# TypeScript
npm run lint
npm run test
```

---

## Coding Standards

### Rust

- Follow `rustfmt` defaults (enforced by CI).
- All public items must have doc comments (`///`).
- Prefer `anyhow::Result` for error propagation in binary code; `thiserror` for library crates.
- No `unwrap()` in library code — use `?` or explicit error handling.

### TypeScript

- Strict mode enabled (`"strict": true` in `tsconfig.json`).
- All exported functions must have explicit return types.
- Use `async/await` over raw Promises.

---

## Submitting a Pull Request

- Keep PRs focused — one logical change per PR.
- Reference the related issue: `Closes #<issue-number>`.
- Fill in the PR template (title, summary, testing notes).
- PRs require at least one approving review before merge.
- Squash-merge is preferred to keep `main` history clean.

---

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Small, well-scoped — ideal for first-time contributors |
| `help wanted` | Maintainers welcome external contributions |
| `enhancement` | New feature or improvement |
| `bug` | Something is broken |
| `documentation` | Docs-only change |
| `infrastructure` | CI, tooling, scripts |
| `rust` | Rust crate work |
| `typescript` | TypeScript package work |
| `blocked` | Waiting on another issue or external dependency |
