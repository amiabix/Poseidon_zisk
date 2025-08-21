# Poseidon Hasher for ZisK

A Rust implementation of the Poseidon hash function designed to work with the ZisK zero-knowledge proof system.

## Overview

This project implements a custom Poseidon hash function that can be used within ZisK circuits. The implementation includes:

- Custom field arithmetic operations
- Simplified Poseidon hash algorithm
- Integration with ZisK for zero-knowledge proof generation
- Build system for generating input files

## Features

- **Field Operations**: Modular arithmetic with a Mersenne prime field modulus
- **Poseidon Hash**: Custom implementation with S-box and mixing layers
- **ZisK Integration**: Built to work with the ZisK zero-knowledge proof system
- **Configurable Iterations**: Supports variable number of hash iterations

## Project Structure

```
poseidon_hasher/
├── src/
│   └── main.rs          # Main ZisK circuit implementation
├── build.rs             # Build script for input generation
├── Cargo.toml           # Rust dependencies and project configuration
├── proof/               # Generated proofs and results
└── build/               # Build artifacts (generated input files)
```

## Dependencies

- `ziskos`: ZisK zero-knowledge proof system
- `byteorder`: Byte order conversion utilities

## Building

To build the project:

```bash
cargo build --release
```

## Usage

The main circuit takes a number `n` as input and computes the Poseidon hash `n` times sequentially.

### Input Format

The input should be an 8-byte little-endian representation of a u64 integer.

### Output Format

The output consists of 8 32-bit values representing the final hash result.

## Implementation Details

### Field Arithmetic

- **Field Modulus**: Uses 2^63 - 1 (Mersenne prime)
- **Addition**: Modular addition with overflow handling
- **Multiplication**: Modular multiplication using 128-bit arithmetic

### Poseidon Hash

- **State Size**: 3 field elements
- **Rounds**: 3 rounds with custom round keys
- **S-box**: x^5 transformation
- **Mixing**: Linear transformation layer

### ZisK Integration

- Uses `#![no_main]` and `ziskos::entrypoint!` macro
- Reads input using `read_input()`
- Sets output using `set_output()`

## Build System

The `build.rs` script generates input files for testing:
- Creates `build/input.bin` with a default value of 20 iterations
- Ensures proper directory structure

## Proof Generation

The project includes generated proofs in the `proof/` directory:
- `result.json`: Proof verification results
- `vadcop_final_proof.bin`: Final proof binary
- `proofs/`: Additional proof artifacts

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please ensure all code follows Rust best practices and includes appropriate tests.
