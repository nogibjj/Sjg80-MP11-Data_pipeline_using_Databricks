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

## Getting Started

**Rust Codespaces** is a Rust Command-Line Interface (CLI) application designed to showcase Rust's capabilities in building efficient and interactive CLI tools. It integrates with an SQLite database, allowing users to perform CRUD operations on a "Customers" table. 

The project demonstrates the structure of a typical Rust CLI application, error handling, SQLite integration, and the utilization of GitHub Copilot for code generation and improvement.

### How to Run the Program

1. **Clone the Repository:**

   Begin by cloning this repository to your local machine:

   ```shell
   git clone https://github.com/nogibjj/Sjg80-IndividualProject2.git

Navigate to the Project Directory:

Change your working directory to the project's root folder:

shell
Copy code
cd Sjg80-IndividualProject2
Build the Rust Binary:

Build the Rust binary by running the following command:

shell
Copy code
cargo build --release
This command compiles the source code and creates an optimized binary in the target/release/ directory.

Run the Binary:

Execute the Rust binary by entering the following command:

shell
Copy code
./target/release/rust-codespaces
Follow the command-line prompts to interact with the SQLite database and perform CRUD operations.

Dependencies and How to Install Them
The project relies on several Rust libraries as dependencies, which are specified in the Cargo.toml file. To install these dependencies, ensure you have Rust and Cargo installed on your system. If you don't, you can install them using the Rust installation guide.

GitHub Actions
The project is integrated with GitHub Actions to automate various development processes. The workflow file, rust.yml, defines the CI/CD pipeline. This workflow performs the following tasks:

Testing of Rust Code:

The workflow runs automated tests for the Rust code to ensure that it behaves correctly.

Building Rust Code:

It compiles the Rust code to create the optimized binary for your project.

Linting of Rust Code:

The code quality and style are checked for adherence to best practices.

These automated processes are triggered on every push to the main branch, ensuring that the code remains reliable and performs as expected.


## How to run the program

To run the program, clone the repository and run the following command:

```
cargo run
```
This will create a new SQLite database and insert a new customer into the database. You can then read, update, and delete customers using the following commands:

```

```

# Video Demo

The following YouTube link shows a clear, concise walkthrough and demonstration of the CLI binary:

[YouTube link]

