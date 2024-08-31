# Caching Proxy server

This project is a web server built using Rust and the Axum framework. It includes functionality to handle HTTP requests and clear cache based on command-line arguments.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

## Installation

Clone the repository:

```sh
git clone https://github.com/doivjpxx/caching-proxy.git <your-project-name>
cd <your-project-name>
```

## Usage

To run the server, use the following command:

```sh
cargo run -- --port <PORT> --origin <ORIGIN>
```

To clear the cache, use the following command:

```sh
cargo run -- --clear-cache
```
---
Project's challenge: [https://roadmap.sh/projects/caching-server](https://roadmap.sh/projects/caching-server)

