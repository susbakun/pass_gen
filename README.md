# Password Generator

A secure password generator written in Rust that creates strong, random passwords with guaranteed character type coverage.

## Features

- **Guaranteed character coverage**: The first 4 characters ensure at least one digit, one lowercase letter, one uppercase letter, and one special character
- **Random generation**: Remaining characters are randomly selected from all character types
- **Configurable length**: Specify password length via command-line argument (default: 12 characters)
- **Input validation**: Ensures minimum length of 4 characters to satisfy all character type requirements
- **Comprehensive validation**: Validates that generated passwords contain all required character types

## Usage

### Basic Usage

Generate a password with the default length (12 characters):

```bash
cargo run
```

### Custom Length

Generate a password with a specific length:

```bash
cargo run 16
```

The length must be at least 4 characters to ensure all character types can be included.

## How It Works

1. **Initial Character Guarantee**: The first 4 characters are guaranteed to include:
   - One digit (0-9)
   - One lowercase letter (a-z)
   - One uppercase letter (A-Z)
   - One special character (!, #, $, %, &, *, ], [, (, ), {, }, +, -)
   
   These characters are shuffled to randomize their positions.

2. **Random Fill**: The remaining positions (if length > 4) are filled with randomly selected characters from all available character types.

3. **Validation**: The generated password is validated to ensure it contains all required character types before being returned.

## Character Sets

- **Digits**: `0-9` (10 characters)
- **Lowercase**: `a-z` (26 characters)
- **Uppercase**: `A-Z` (26 characters)
- **Special**: `!, #, $, %, &, *, ], [, (, ), {, }, +, -` (14 characters)

## Examples

```bash
$ cargo run
Here is the password: K3m!xP9$vL2n

$ cargo run 20
Here is the password: A7#bN2mK9!xP4$vL8nQ3
```

## Testing

Run the test suite:

```bash
cargo test
```

The tests verify:
- Password validation logic
- Password generation with guaranteed character coverage

## Requirements

- Rust 2024 edition or later
- `rand` crate (version 0.9.2)

## Project Structure

```
password_generator/
├── src/
│   ├── main.rs      # Entry point and CLI argument parsing
│   └── passgen.rs   # Password generation logic
├── Cargo.toml       # Project dependencies
└── README.md        # This file
```

## Security Notes

- Uses Rust's `rand` crate with secure random number generation
- Passwords are generated using cryptographically secure randomness
- The guarantee of character type coverage ensures passwords meet common security requirements

