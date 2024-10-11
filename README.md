# zKloud Solana 
zKloud is a next-generation cloud hardware rental platform built on the Solana blockchain. This repository contains the Rust implementation of the Solana program (smart contract) that powers the zKloud platform.

## Features

- Hardware rental initialization
- Rental completion
- Integration with Blinks for payments
- Efficient on-chain storage of rental data

## Prerequisites

- Rust 1.68 or later
- Solana CLI 1.16 or later

## Building

To build the project, run:

```bash
cargo build-bpf
```

## Testing

To run the tests:

```bash
cargo test-bpf
```

## Deployment

To deploy the program to Solana devnet:

```bash
solana program deploy target/deploy/zkloud_solana.so
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
