# Hashassin

Hashassin is a Rust-based command-line tool for generating passwords, hashing them using various algorithms, and dumping hash data. It is designed to be fast, efficient, and highly configurable, leveraging multi-threading and modern cryptographic libraries.

## Features

- **Password Generation**: Generate random passwords with customizable length, number, and output options.
- **Hash Generation**: Hash passwords using algorithms like MD5, SHA-256, SHA-512, and Scrypt.
- **Hash Dumping**: Parse and display hash data from files in a structured format.
- **Multi-threading**: Utilize multiple threads for faster processing.
- **Extensible**: Built with modularity in mind, making it easy to add new features or algorithms.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/Password-Hashing-Rust.git
   cd Password-Hashing-Rust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available in the `target/release` directory.

## Usage

Hashassin provides three main commands: `gen-passwords`, `gen-hashes`, and `dump-hashes`. Below are examples of how to use each command.

### 1. Generate Passwords

Generate random passwords and optionally save them to a file.

```bash
./hashassin gen-passwords --chars 8 --num 100 --threads 4 --out-file passwords.txt
```

- `--chars`: Number of characters per password (default: 4).
- `--num`: Number of passwords to generate.
- `--threads`: Number of threads to use (default: 1).
- `--out-file`: Path to save the generated passwords (optional).

### 2. Generate Hashes

Hash passwords from an input file and save the hashes to an output file.

```bash
./hashassin gen-hashes --in-file passwords.txt --out-file hashes.txt --threads 4 --algorithm sha256
```

- `--in-file`: Path to the input file containing passwords.
- `--out-file`: Path to save the generated hashes.
- `--threads`: Number of threads to use (default: 1).
- `--algorithm`: Hashing algorithm to use (`md5`, `sha256`, `sha512`, or `scrypt`).

### 3. Dump Hashes

Parse and display hash data from a file.

```bash
./hashassin dump-hashes --in-file hashes.txt
```

- `--in-file`: Path to the file containing hash data.

## Examples

### Generate 10 passwords of 12 characters each and save to `passwords.txt`:
```bash
./hashassin gen-passwords --chars 12 --num 10 --out-file passwords.txt
```

### Hash passwords using SHA-512 and save to `hashes.txt`:
```bash
./hashassin gen-hashes --in-file passwords.txt --out-file hashes.txt --algorithm sha512
```

### Dump and display hash data from `hashes.txt`:
```bash
./hashassin dump-hashes --in-file hashes.txt
```

## Configuration

The tool uses the `RUST_LOG` environment variable for logging. To enable detailed logs, set the environment variable before running the tool:

```bash
export RUST_LOG=trace
```

## Development

### Prerequisites

- Rust (edition 2024)
- Cargo

### Build and Run

1. Build the project:
   ```bash
   cargo build
   ```

2. Run the tool:
   ```bash
   cargo run -- <command> [options]
   ```

### Testing

Run the tests using:
```bash
cargo test
```

## Project Structure

```
Password-Hashing-Rust/
├── cli/                # Command-line interface
│   ├── src/
│   │   ├── main.rs     # Entry point for the CLI
│   │   ├── gen_passwords.rs # Password generation logic
│   │   ├── gen_hashes.rs    # Hash generation logic
│   │   ├── dump_hashes.rs   # Hash dumping logic
│   └── Cargo.toml      # CLI dependencies
├── core/               # Core library
│   ├── src/
│   │   ├── lib.rs      # Core library entry point
│   │   ├── args.rs     # Argument parsing and enums
│   │   ├── utility.rs  # Utility functions and traits
│   └── Cargo.toml      # Core library dependencies
├── Cargo.toml          # Workspace configuration
└── README.md           # Project documentation
```

## Dependencies

- [Clap](https://crates.io/crates/clap): Command-line argument parsing.
- [Rand](https://crates.io/crates/rand): Random number generation.
- [Sha2](https://crates.io/crates/sha2): SHA-256 and SHA-512 hashing.
- [Sha3](https://crates.io/crates/sha3): SHA-3 hashing.
- [Scrypt](https://crates.io/crates/scrypt): Scrypt password hashing.
- [Tracing](https://crates.io/crates/tracing): Logging and diagnostics.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

---

Happy hashing! 🎉