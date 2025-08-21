// This example program takes a number `n` as input and computes the Poseidon hash `n` times sequentially.

// Mark the main function as the entry point for ZisK
#![no_main]
ziskos::entrypoint!(main);

use std::convert::TryInto;
use ziskos::{read_input, set_output};
use byteorder::ByteOrder;

// Custom Poseidon hash implementation for ZisK
// Using a simplified version with basic field operations

// Field modulus for a smaller field (using a prime < 2^64)
const FIELD_MODULUS: u64 = 0x7fffffffffffffff; // 2^63 - 1 (Mersenne prime)

// Simple field addition with modular reduction
fn field_add(a: u64, b: u64) -> u64 {
    let sum = a as u128 + b as u128;
    if sum >= FIELD_MODULUS as u128 {
        (sum - FIELD_MODULUS as u128) as u64
    } else {
        sum as u64
    }
}

// Simple field multiplication with modular reduction
fn field_mul(a: u64, b: u64) -> u64 {
    let product = (a as u128) * (b as u128);
    (product % FIELD_MODULUS as u128) as u64
}

// S-box function: x^5 (mod field_modulus)
fn sbox(x: u64) -> u64 {
    let x2 = field_mul(x, x);
    let x4 = field_mul(x2, x2);
    field_mul(x4, x)
}

// Simple round function for Poseidon
fn poseidon_round(state: &mut [u64; 3], round_key: u64) {
    // Add round key
    state[0] = field_add(state[0], round_key);
    
    // Apply S-box to first element
    state[0] = sbox(state[0]);
    
    // Mix layer (simple matrix multiplication)
    let temp0 = state[0];
    let temp1 = state[1];
    let temp2 = state[2];
    
    state[0] = field_add(field_add(temp0, temp1), temp2);
    state[1] = field_add(field_add(temp0, field_mul(temp1, 2)), temp2);
    state[2] = field_add(temp0, field_add(temp1, field_mul(temp2, 3)));
}

// Custom Poseidon hash function
fn poseidon_hash(input: &[u8; 32]) -> [u8; 32] {
    // Convert input bytes to field elements
    let mut state = [0u64; 3];
    
    // Simple conversion: take 8 bytes at a time and convert to u64
    for i in 0..3 {
        let start = i * 8;
        let end = start + 8;
        if end <= 32 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&input[start..end]);
            state[i] = u64::from_le_bytes(bytes) % FIELD_MODULUS;
        }
    }
    
    // Apply rounds (simplified: just 3 rounds)
    let round_keys = [0x1234567890abcdef, 0xfedcba0987654321, 0xdeadbeefcafebabe];
    
    for &round_key in &round_keys {
        poseidon_round(&mut state, round_key);
    }
    
    // Convert back to bytes
    let mut output = [0u8; 32];
    for i in 0..3 {
        let start = i * 8;
        let end = start + 8;
        if end <= 32 {
            output[start..end].copy_from_slice(&state[i].to_le_bytes());
        }
    }
    
    output
}

fn main() {
    // Read the input data as a byte array from ziskos
    let input: Vec<u8> = read_input();

    // Convert the input data to a u64 integer
    let n: u64 = match input.try_into() {
        Ok(input_bytes) => u64::from_le_bytes(input_bytes),
        Err(input) => panic!("Invalid input length. Expected 8 bytes, got {}", input.len()),
    };
    
    // Start with a default hash value (32 bytes of zeros)
    let mut hash = [0u8; 32];

    // Compute Poseidon hashing 'n' times
    for _ in 0..n {
        // Hash the current hash value using our custom Poseidon
        hash = poseidon_hash(&hash);
    }

    // Split 'hash' value into chunks of 32 bits and write them to ziskos output
    for i in 0..8 {
        let val = byteorder::BigEndian::read_u32(&mut hash[i * 4..i * 4 + 4]);
        set_output(i, val);
    }
}