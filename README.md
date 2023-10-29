# Sjg80-RUSTbasic

# Rust CLI Binary with SQLite

This Rust project showcases a Command-Line Interface (CLI) binary that interacts with an SQLite database, allowing you to perform CRUD (Create, Read, Update, Delete) operations on a "Customers" table.

## Introduction

This Rust CLI binary is designed to work with an SQLite database that contains a "Customers" table. You can create, read, update, and delete customer records through the CLI. The project leverages the `rusqlite` crate for database interaction.

## Usage

Follow the steps below to set up and use this Rust CLI binary:

### Prerequisites

- Rust programming environment: Make sure you have Rust installed on your system.

### Dependencies

This project uses the following dependencies:

- `rusqlite`: For SQLite database interaction.
- `csv`: To work with CSV files.
- `reqwest`: To make HTTP requests.
- `assert_cmd`: For testing command-line applications.
- `predicates`: For testing command-line output.
- `libc`: For C standard library bindings.
- `rust-bert`: For natural language processing (NLP).

To install the Rust dependencies, add them to your `Cargo.toml` file and run `cargo build`.

### Building the Binary

To build the Rust binary, use the following command:

```bash
cargo build --release
