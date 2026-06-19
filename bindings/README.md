# TypeScript Bindings

This directory is reserved for generated TypeScript bindings from the Soroban contract interface.
Generate bindings after building the contract and place them in this folder.

```bash
soroban contract bindings ts --wasm target/wasm32-unknown-unknown/release/sorobanchron-contracts.wasm --out bindings
```
