# work-tokens

A blockchain-based platform that issues work tokens as a form of currency for completed work. The system is designed to prevent wealth accumulation by ensuring tokens cannot be hoarded, promoting equitable distribution and continuous participation.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Introduction

**Work Tokens** is a decentralised application built in Rust, focusing on creating an equitable economic system where tokens are earned through verified work and are subject to expiration to prevent hoarding. This approach aims to address wealth inequality by tying value directly to ongoing contributions rather than accumulation.

## Features

- **Work-Based Token Issuance**: Earn tokens by completing and verifying work tasks.
- **Token Expiration Mechanism**: Tokens expire after a set period to prevent hoarding.
- **Use-It-or-Lose-It Policy**: Encourages active participation and circulation of tokens.
- **Decentralised Consensus**: Secure and efficient consensus mechanism for validating transactions and blocks.
- **High Performance and Safety**: Built with Rust, ensuring memory safety and concurrency without sacrificing performance.
- **Peer-to-Peer Networking**: Decentralised P2P network for node communication.
- **Cryptographic Security**: Utilises modern cryptographic techniques for data integrity and authentication.

## Prerequisites

- **Rust**: You need to have Rust installed (version 1.50 or later). You can install it using `rustup`:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: Rust's package manager (installed with Rust).

- **Git**: For cloning the repository.

## Installation

1. **Clone the Repository**

   ```bash
   git clone git@github.com:starbornone/work-tokens.git
   cd work-tokens
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

   This will compile the project in release mode for optimal performance.

## Usage

### Running a Node

To start a node on the Work Tokens Blockchain network:

```bash
cargo run --release -- --port 3030 --peer-address <peer_node_address>
```

Replace `<peer_node_address>` with the address of a peer node you wish to connect to.

### Command-Line Options

- `--port`: Specify the port on which the node will listen (default: 3030).
- `--peer-address`: The address of a peer node to connect with at startup.
- `--config`: Path to a configuration file (optional).
- `--help`: Display help information about command-line options.

### Submitting Work and Earning Tokens

To simulate submitting work and earning tokens, use the wallet CLI (to be implemented):

```bash
cargo run --release -- wallet submit-work --details "Completed task XYZ"
```

### Checking Balance

```bash
cargo run --release -- wallet balance
```

### Transferring Tokens

```bash
cargo run --release -- wallet transfer --to <recipient_address> --amount <token_amount>
```

## Project Structure

```plaintext
work-tokens/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── blockchain/
│   │   ├── mod.rs
│   │   ├── block.rs
│   │   ├── transaction.rs
│   │   ├── ledger.rs
│   │   └── merkle_tree.rs
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── proof_of_work.rs
│   │   └── validator.rs
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── hash.rs
│   │   ├── keys.rs
│   │   └── signatures.rs
│   ├── network/
│   │   ├── mod.rs
│   │   ├── p2p.rs
│   │   └── message.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── db.rs
│   ├── token/
│   │   ├── mod.rs
│   │   ├── issuance.rs
│   │   ├── expiration.rs
│   │   └── management.rs
│   ├── wallet/
│   │   ├── mod.rs
│   │   └── wallet.rs
│   ├── api/
│   │   ├── mod.rs
│   │   └── rpc.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   └── utils/
│       ├── mod.rs
│       └── serialization.rs
├── tests/
│   ├── integration_tests.rs
│   └── blockchain_tests.rs
├── scripts/
│   ├── start_node.sh
│   └── run_tests.sh
└── docs/
    ├── design.md
    ├── api.md
    └── README.md
```

- **`src/`**: Contains all Rust source code, organised into modules.
- **`tests/`**: Contains integration and unit tests.
- **`scripts/`**: Scripts for automating tasks like starting nodes or running tests.
- **`docs/`**: Documentation files, including design documents and API references.

## Contributing

Contributions are welcome! To contribute:

1. **Fork the Repository**

   Click the "Fork" button at the top-right corner of the repository page.

2. **Clone Your Fork**

   ```bash
   git clone git@github.com:starbornone/work-tokens.git
   cd work-tokens
   ```

3. **Create a Feature Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**

   Implement your feature or fix.

5. **Commit Your Changes**

   ```bash
   git commit -am 'Add new feature'
   ```

6. **Push to Your Fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Open a Pull Request**

   Go to the original repository and open a pull request from your fork.
