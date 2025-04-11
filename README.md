# gen-bls-key

A command-line tool for generating BLS12-381 keys and deriving public keys from private keys.

## ⚠️ Warning

This package is minimally tested and should be used with caution. While it includes basic unit tests, it has not undergone extensive security audits or testing in production environments. Use at your own risk.

## Features

- Generate new BLS12-381 key pairs (private and public keys)
- Derive public keys from existing private keys
- Support for hex-encoded private keys (with or without 0x prefix)

## Installation

### From crates.io (recommended)

```bash
cargo install gen-bls-key
```

### From source

```bash
# Clone the repository
git clone https://github.com/yourusername/gen-bls-key.git
cd gen-bls-key

# Build the project
cargo build --release
```

## Usage

### Generate a new key pair

To generate a new BLS12-381 key pair (private and public keys):

```bash
gen-bls-key
```

Example output:

```
Private key: 0x1234...  # 32 bytes
Public key: 0x5678...   # 48 bytes
```

### Derive public key from private key

To derive a public key from an existing private key:

```bash
gen-bls-key --private-key 0x1234...  # Replace with your private key
# or
gen-bls-key -p 0x1234...  # Short form
```

Example output:

```
Public key: 0x5678...   # 48 bytes
```

### Command-line options

```
A tool for generating BLS12-381 keys

Usage: gen-bls-key [OPTIONS]

Options:
  -p, --private-key <PRIVATE_KEY>  Private key in hex format (with or without 0x prefix)
  -h, --help                       Print help
  -V, --version                    Print version
```

## Key Formats

- Private keys are 32 bytes long and are output in hex format
- Public keys are 48 bytes long (compressed) and are output in hex format
- Private keys can be provided with or without the "0x" prefix

## Development

### Running tests

```bash
cargo test
```

### Building for release

```bash
cargo build --release
```

The binary will be available at `target/release/gen-bls-key`.
