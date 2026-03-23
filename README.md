[![Build](https://github.com/lukecampbell/genpass/actions/workflows/build.yml/badge.svg)](https://github.com/lukecampbell/genpass/actions/workflows/build.yml)

genpass
===============

Quick password generation (now in Rust)

Copyright 2026 Axiom Data Science, LLC

See LICENSE for details.

Usage
-----

    genpass [OPTIONS]

Options:

    -n, --length <LENGTH>    Password length [default: 32]
    -s, --secure             Include all special characters (~!@#$%^&*()_+`-=[]{};':",./<>?\|)
    -u, --semi-secure        Include safer special characters only (_+-=?)
    -h, --help               Print help
    -V, --version            Print version

Examples:

    genpass                        # 32-character alphanumeric password
    genpass -n 64                  # 64-character alphanumeric password
    genpass --secure               # 32-character password with all special characters
    genpass -n 16 --semi-secure    # 16-character password with safer special characters

Building
--------

Requires Rust — see [Install Rust](https://www.rust-lang.org/tools/install) for details.

    cargo build --release

The compiled binary will be at `target/release/genpass`.

To run without installing:

    cargo run -- [OPTIONS]

For more on `cargo`, see [The Cargo Book](https://doc.rust-lang.org/cargo/commands/index.html).
