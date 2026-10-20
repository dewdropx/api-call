# GitHub Contributors API Client (Rust)

A simple Rust project that demonstrates how to make an **asynchronous HTTP GET request** to the GitHub REST API using `reqwest` and `tokio`. The project retrieves contributor information for a GitHub repository, deserializes the JSON response using `serde`, and displays each contributor's username and ID.

## Features

- **Asynchronous HTTP requests** using `reqwest` and Tokio.
- **JSON deserialization** using Serde.
- **HTTP status handling** to check whether a request succeeds.
- **Error handling** using `Result` and Rust's `?` operator.
- **Custom HTTP headers** using the `USER_AGENT` header.
- **Contributor data extraction** using Rust structs and vectors.

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/) — HTTP client.
- [Tokio](https://tokio.rs/) — Asynchronous runtime.
- [Serde](https://serde.rs/) — Serialization and deserialization framework.

## API Endpoint

The project uses the GitHub REST API to retrieve contributors for the following repository:

`https://api.github.com/repos/rust-lang-nursery/rust-cookbook/contributors?per_page=100`

The `per_page=100` parameter requests up to 100 contributors in a single response.

## Data Structure

The `User` struct represents the relevant fields returned by the GitHub API:

```rust
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u64,
}
```

## Prerequisites

Install Rust and Cargo using [rustup](https://rustup.rs/).

## Installation and Usage

**1. Clone the repository**

```bash
git clone https://github.com/dewdropx/api-call.git
cd api-call
```

**2. Configure dependencies**

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.13", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

**3. Run the project**

```bash
cargo run
```

## Example Output

```text
https://api.github.com/repos/rust-lang-nursery/rust-cookbook/contributors?per_page=100
status: 200 OK
login: example-user, id: 123456
login: another-user, id: 654321
```

The contributor usernames and IDs above are illustrative. Actual output depends on the API response.
