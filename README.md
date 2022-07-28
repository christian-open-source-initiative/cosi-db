# COSI Database

A database used for capturing ministry and community life within, usually, a church setting!

## Requirements

* Rust `1.62.1`
* MongoDB `v6.0.0`

## Build and Setup

### Docker

Only available on Linux platforms. Windows and Mac, try at your own risk as subnetworking can be inconsistent.
You should have docker and docker compose plugin installed.

```bash
docker compose up
```

Then navigate to `127.0.0.1:8000`.

**Note that docker deployment is useful for testing. However, can be noticeably slow. Docker runs in debug build by default.**

### Native Install

You must have the proper MongoDB version installed. Then:

```bash
cargo run
```

## Develop

### Setup Auto Formatting

Our team uses `cargo fmt` to format incoming code.
Please execute the following to set-up auto formatting:

```bash
tools/install.py
```

This will setup the necessary scripts for your local development environment.
