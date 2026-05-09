# farmledge-protocol

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Build](https://github.com/farmledge-labs/farmledge-protocol/actions/workflows/test.yml/badge.svg)](https://github.com/farmledge-labs/farmledge-protocol/actions/workflows/test.yml)
[![Audit](https://github.com/farmledge-labs/farmledge-protocol/actions/workflows/audit.yml/badge.svg)](https://github.com/farmledge-labs/farmledge-protocol/actions/workflows/audit.yml)

Open protocol for tokenizing physical agricultural commodity deposits on [Stellar](https://stellar.org). Soroban smart contracts, a token metadata standard, and a TypeScript SDK for warehouse receipt tokenization across Nigerian agricultural markets.

---

## Overview

farmledge-protocol enables licensed warehouse custodians to issue on-chain warehouse receipts representing physical commodity deposits. Each receipt is a Soroban token carrying structured metadata — commodity type, quantity, grade, custodian identity, and expiry — making it tradeable, financeable, and auditable without leaving the Stellar network.

**Current commodities:** Maize · Sesame

---

## Repository Structure

```
farmledge-protocol/
├── contracts/
│   ├── maize-receipt/      # Soroban contract — maize warehouse receipts
│   └── sesame-receipt/     # Soroban contract — sesame warehouse receipts
├── sdk/                    # TypeScript SDK for dApp and backend integration
├── scripts/                # Deployment and account-funding scripts
├── docs/
│   ├── TOKEN_STANDARD.md   # Token metadata specification
│   ├── GRADING_STANDARDS.md
│   └── CUSTODIAN_GUIDE.md
└── .github/
    ├── workflows/          # CI: test + audit
    └── ISSUE_TEMPLATE/
```

---

## Contract Addresses

| Contract        | Network  | Address |
|-----------------|----------|---------|
| maize-receipt   | Testnet  | TBD     |
| maize-receipt   | Mainnet  | TBD     |
| sesame-receipt  | Testnet  | TBD     |
| sesame-receipt  | Mainnet  | TBD     |

---

## Getting Started

**Prerequisites**

- Rust stable + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- Node.js 20+

**Build contracts**

```sh
rustup target add wasm32-unknown-unknown
cargo build --workspace --target wasm32-unknown-unknown --release
```

**Run tests**

```sh
cargo test --workspace
```

**Install SDK**

```sh
cd sdk && npm install
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [Token Standard](docs/TOKEN_STANDARD.md) | Metadata fields and token lifecycle |
| [Grading Standards](docs/GRADING_STANDARDS.md) | Commodity grade codes and inspection process |
| [Custodian Guide](docs/CUSTODIAN_GUIDE.md) | Onboarding and operational guide for custodians |

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. For security issues, see [SECURITY.md](SECURITY.md).

---

## License

Apache 2.0 — see [LICENSE](LICENSE). Copyright 2026 Farmledge Labs.
