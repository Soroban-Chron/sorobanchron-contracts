# sorobanchron-contracts

A smart contract repository for the SorobanChron automation registry, keeper rewards, and on-chain governance hooks.

## What's in this repo

- `src/` — Rust-based Soroban contract implementation for job registration, execution validation, and payout logic.
- `Cargo.toml` — package configuration with Soroban SDK dependency and wasm build settings.
- `.gitignore` — ignores build artifacts, WASM binaries, and lockfiles.
- `README.md` — this project overview and developer guide.

## Architecture

```
 ┌───────────────────────────────┐
 │      sorobanchron-frontend    │◀─── Gets jobs & metrics
 └──────────────┬────────────────┘
                │
                │ (User signs transaction via Freighter)
                ▼
 ┌───────────────────────────────┐       ┌───────────────────────────────┐
 │     sorobanchron-contracts    │◄──────┤      sorobanchron-keeper      │
 │ - Deployed on Stellar network │       │ - Listens to ledger streams   │
 │ - Holds job registry & funds  │       │ - Dispatches competitive txs  │
 └───────────────────────────────┘       └───────────────────────────────┘
```

## For developers: fork & clone

If you plan to contribute, fork this repository on GitHub and clone your fork locally:

```bash
git clone https://github.com/<your-org>/sorobanchron-contracts.git
cd sorobanchron-contracts
git remote add upstream https://github.com/Soroban-Chron/sorobanchron-contracts.git
```

If you only want to run the repo locally, clone directly:

```bash
git clone https://github.com/Soroban-Chron/sorobanchron-contracts.git
cd sorobanchron-contracts
```

## Prerequisites

- **Rust 1.72+**
- **Cargo**
- **Soroban CLI** installed and configured
- **WASM target** added: `rustup target add wasm32-unknown-unknown`

## Run locally

To build and test locally, run:

```bash
cargo build --target wasm32-unknown-unknown
cargo test
```

## Continuous integration

The project runs `cargo build --target wasm32-unknown-unknown` and `cargo test` on every push and pull request via GitHub Actions.


To build the deployable WASM and verify contract behavior:

```bash
soroban contract build
```

## Deploy

```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/sorobanchron-contracts.wasm --network testnet
```

## Local services

This repository is purely contract code and does not run a local app server.
Use companion repos for frontend dashboards and keeper bot infrastructure.

## TypeScript bindings

Generate TypeScript bindings so `sorobanchron-frontend` and `sorobanchron-keeper` can consume the contract interface safely:

```bash
soroban contract bindings ts --wasm target/wasm32-unknown-unknown/release/sorobanchron-contracts.wasm --out bindings
```

## About

This repo contains the core on-chain logic for SorobanChron. It is intentionally decoupled from frontend and keeper implementation details so the smart contract can evolve independently.

## License

MIT OR Apache-2.0
