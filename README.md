# SoroForge CLI

> Hardhat-like ergonomics for Soroban smart contract developers.

[![CI](https://github.com/Stellar-Project-Hub/SoroForge-CLI/actions/workflows/ci.yml/badge.svg)](https://github.com/Stellar-Project-Hub/SoroForge-CLI/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Node.js](https://img.shields.io/badge/node.js-6DA55F?logo=node.js&logoColor=white)](https://nodejs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

SoroForge is a single CLI tool that gives Soroban developers the same productive workflow Hardhat gave Ethereum developers — a local sandbox, hot-reload on save, and one-command testnet deployments.

---

## Features

| Feature | Status |
|---|---|
| `soroforge init` — scaffold a new project | ✅ |
| `soroforge sandbox` — local Soroban node via Docker | 🚧 |
| `soroforge deploy` — one-command testnet/mainnet deploy | 🚧 |
| `soroforge watch` — hot-reload on contract file changes | 🚧 |
| TypeScript SDK (`@soroforge/sdk`) | 🚧 |

---

## Installation

```bash
cargo install soroforge
```

Or build from source:

```bash
git clone https://github.com/Stellar-Project-Hub/SoroForge-CLI.git
cd SoroForge-CLI
cargo build --release
# binary at ./target/release/soroforge
```

---

## Quick Start

```bash
# 1. Create a new project
soroforge init my_project && cd my_project

# 2. Start a local sandbox
soroforge sandbox --port 8000

# 3. Deploy to testnet
soroforge deploy --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
                 --name my_contract \
                 --network testnet

# 4. Watch for changes (hot-reload)
soroforge watch --dir contracts
```

---

## Repository Structure

```
SoroForge-CLI/
├── cli/                        # Main binary crate (soroforge)
│   └── src/
│       ├── main.rs             # CLI entry point, clap command tree
│       └── commands/           # One file per subcommand
│           ├── sandbox.rs
│           ├── deploy.rs
│           ├── watch.rs
│           └── init.rs
│
├── crates/                     # Rust library crates
│   ├── soroforge-core/         # Shared types, config, error types
│   ├── soroforge-sandbox/      # Local sandbox lifecycle (Docker)
│   └── soroforge-deploy/       # Contract upload & instantiation
│
├── packages/                   # TypeScript/Node.js packages
│   ├── sdk/                    # @soroforge/sdk — TS client library
│   └── hot-reload/             # @soroforge/hot-reload — file watcher
│
├── tests/
│   └── integration/            # End-to-end integration tests
│
├── docs/                       # Extended documentation
├── scripts/                    # Helper shell scripts
├── .github/
│   └── workflows/
│       └── ci.yml              # Lint + test on every PR
├── Cargo.toml                  # Workspace manifest
├── package.json                # npm workspace manifest
└── soroforge.example.toml      # Example project config
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get involved.

---

## License

MIT © [Stellar-Project-Hub](https://github.com/Stellar-Project-Hub)
