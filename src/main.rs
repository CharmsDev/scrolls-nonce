use bitcoin::hashes::{Hash, sha256};
use std::env;

/// Calculate Scrolls nonce from funding UTXO and output index.
///
/// Formula: nonce = sha256("{funding_utxo_id}:{output_index}")
///          truncated to first 8 bytes, converted to u64 little-endian
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <funding_utxo_id> <output_index>", args[0]);
        eprintln!("Example: {} abc123...def:0 0", args[0]);
        std::process::exit(1);
    }

    let funding_utxo = &args[1];
    let output_index = &args[2];

    let input = format!("{}:{}", funding_utxo, output_index);
    let hash = sha256::Hash::hash(input.as_bytes());
    let hash_bytes = hash.as_byte_array();

    // Take first 8 bytes, convert to u64 little-endian
    let nonce = u64::from_le_bytes(hash_bytes[0..8].try_into().unwrap());

    println!("{}", nonce);
}
