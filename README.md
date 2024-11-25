## Solana Optimized Native Rust Q4 Programs

The collection of Native Rust Solana programs (unsafe Rust) using [Pinocchio](https://crates.io/crates/pinocchio) as an alternative for [solana-program](https://crates.io/crates/solana-program), [solana-nostd-sha256](https://crates.io/crates/solana-nostd-sha256) for finding PDAs and [five8-const](https://crates.io/crates/five8_const) for constant memory access of the program ID. Program tests are written using [Mollusk](https://crates.io/crates/mollusk-svm) in Rust to directly invoke loaded executables using BPF loader for comprehensive program testing.

### Vault

A simple vault program that allows a user to deposit and withdraw SOL.

### Escrow

An escrow program that acts as a trustless intermediary for atomic token swaps between two parties.

### Fundraiser

A fundraiser program that allows a user to create a fundraiser, receive funds, withdraw funds, and refund funds.

### AMM

An AMM program that allows users to deposit, lock, swap and withdraw tokens from liquidity pools.

### SBPF Close

A super efficient Solana account closing script written in sbpf assembly.

### Notes

- Solana development using Anchor is very convenient and beginner-friendly, but doesn't give us control over program logic at granular level and optimizations.
- Solana native Rust development is great and suggested to use for production.
- These optimizations uses unsafe Rust and crates that are not fully audited but showcases potential optimizations over existing Anchor and native development.
- While Solana smart contracts development imposes a lot of overhead on the developer side, these optimizations impose more overhead on the client side.
