# scrolls-nonce

A command-line utility to calculate [Scrolls](https://scrolls-v9.charms.dev) nonces from funding UTXOs.

## What it does

Calculates a nonce value by:
1. Taking a funding UTXO ID and output index
2. Computing SHA256 hash of `{funding_utxo_id}:{output_index}`
3. Extracting the first 8 bytes and converting to a u64 (little-endian)

## Installation

```bash
cargo install --path .
```

## Usage

```bash
scrolls-nonce <funding_utxo_id> <output_index>
```

### Example

```bash
scrolls-nonce abc123def456:0 0
```

This will output a numeric nonce value.

## License

MIT
