# p2p-gossping

## Installation

1. Install Rust

2. Build Project

```sh
cargo build
```

## Usage

1. Start the first peer
```sh
cargo run -- --period=5 --port=8080
```

2. Start the next peer
```sh
cargo run -- --period=5 --port="other port number" --connect=8080
