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


# Rust CLI Binary with SQLite

This project demonstrates how to create a Rust CLI binary with a SQLite database. The binary can be used to perform CRUD operations on the database.

# How to use GitHub Copilot

GitHub Copilot is a tool that can be used to generate Rust code. To use GitHub Copilot in this project, I installed the GitHub Copilot extension for Visual Studio Code. Once the extension was installed, I could generate Rust code by typing `// <tab>`.

# SQLite Database

The SQLite database is a lightweight database that can be used to store data. To use the SQLite database in this project, I installed the `rusqlite` crate. The `rusqlite` crate provides a Rust interface to the SQLite database.

# CRUD Operations

The following CRUD operations are demonstrated in this project:

* **Create:** The `main()` function creates a new customer in the database.
* **Read:** The `main()` function reads all customers from the database.
* **Update:** The `main()` function updates the household income of a customer in the database.
* **Delete:** The `main()` function deletes a customer from the database.

# Optimized Rust Binary

The following process is used to generate an optimized Rust binary:

1. The `cargo build --release` command is used to build the Rust binary in release mode.
2. The `strip` command is used to remove unnecessary symbols from the Rust binary.
3. The `upx` command is used to compress the Rust binary.

The optimized Rust binary is generated as a GitHub Actions artifact that can be downloaded.

## Dependencies

The following dependencies are required to run this project:

* `rusqlite`
* `cargo`
* `strip`
* `upx`

## How to run the program

To run the program, clone the repository and run the following command:

```
cargo run
```
This will create a new SQLite database and insert a new customer into the database. You can then read, update, and delete customers using the following commands:

```
# Read all customers
cargo run -- read

# Update the household income of a customer
cargo run -- update --customer-id 1 --household-income 85000

# Delete a customer
cargo run -- delete --customer-id 1
```

# Video Demo

The following YouTube link shows a clear, concise walkthrough and demonstration of the CLI binary:

[YouTube link]

