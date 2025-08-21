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

**Note**: This is a ZisK-specific project that compiles to RISC-V architecture and generates zero-knowledge proofs.

## Prerequisites

Before building and running this project, ensure you have:

- **Rust**: Latest stable version
- **cargo-zisk**: ZisK CLI tool for building and proving
- **ZisK**: Zero-knowledge proof system installation
- **MPI**: For concurrent proof generation (optional)
- **CUDA Toolkit**: For GPU support (optional, NVIDIA only)

## Dependencies

- `ziskos`: ZisK proof system
- `byteorder`: Byte order conversion utilities

## Building

To build the project for ZisK:

```bash
# Build for ZisK (RISC-V architecture)
cargo-zisk build --release

# The resulting ELF file will be generated in:
# ./target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher
```

## Usage

The main circuit takes a number `n` as input and computes the Poseidon hash `n` times sequentially.

### Input Format

The input should be an 8-byte little-endian representation of a u64 integer.

### Output Format

The output consists of 8 32-bit values representing the final hash result.

## Execution and Proof Generation

### Test with ZisK Emulator

Before generating a proof, test your compiled program using the ZisK emulator:

```bash
# Build and execute in one command
cargo-zisk run --release -i build/input.bin

# Or build first, then execute separately
cargo-zisk build --release
ziskemu -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -i build/input.bin

# If you encounter step limit errors, increase max steps
ziskemu -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -i build/input.bin -n 10000000000
```

### Generate Program Setup

First time setup or after rebuilding:

```bash
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -k $HOME/.zisk/provingKey
```

### Verify Constraints

Verify that all constraints are satisfied:

```bash
cargo-zisk verify-constraints -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -i build/input.bin -w $HOME/.zisk/bin/libzisk_witness.so -k $HOME/.zisk/provingKey
```

### Generate Proof

Generate the zero-knowledge proof:

```bash
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -i build/input.bin -w $HOME/.zisk/bin/libzisk_witness.so -k $HOME/.zisk/provingKey -o proof -a -y
```

### Verify Proof

Verify the generated proof:

```bash
cargo-zisk verify -p ./proof/vadcop_final_proof.bin -s $HOME/.zisk/provingKey/zisk/vadcop_final/vadcop_final.starkinfo.json -e $HOME/.zisk/provingKey/zisk/vadcop_final/vadcop_final.verifier.bin -k $HOME/.zisk/provingKey/zisk/vadcop_final/vadcop_final.verkey.json
```

### Performance Metrics

Get execution metrics and statistics:

```bash
# Execution metrics
cargo-zisk run --release -i build/input.bin -m

# Execution statistics
cargo-zisk run --release -i build/input.bin -x
```

### Advanced Proof Generation

#### Concurrent Proof Generation

For improved performance, you can generate proofs using multiple processes:

```bash
mpirun --bind-to none -np <num_processes> -x OMP_NUM_THREADS=<num_threads_per_process> -x RAYON_NUM_THREADS=<num_threads_per_process> target/release/cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/poseidon_hasher -i build/input.bin -o proof -a -y
```

**Note:** `<num_processes>` × `<num_threads_per_process>` should match your available CPU cores.

#### GPU Support

For NVIDIA GPUs, build ZisK with GPU support:

```bash
cargo build --release --features gpu
```

GPU support can be combined with concurrent proof generation for maximum performance.

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

### ZisK Integration

The project is specifically designed for ZisK:
- Uses `#![no_main]` and `ziskos::entrypoint!` macro
- Compiles to RISC-V architecture for ZisK execution
- Integrates with ZisK's input/output system using `read_input()` and `set_output()`
- Generates zero-knowledge proofs for computational integrity

## Proof Generation

The project includes generated proofs in the `proof/` directory:
- `result.json`: Proof verification results
- `vadcop_final_proof.bin`: Final proof binary
- `proofs/`: Additional proof artifacts

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please ensure all code follows Rust best practices and includes appropriate tests.
