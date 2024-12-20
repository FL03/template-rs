# {{ project-name }}

[![crates.io](https://img.shields.io/crates/v/{{ project-name }}.svg)](https://crates.io/crates/{{ project-name }})
[![docs.rs](https://docs.rs/{{ project-name }}/badge.svg)](https://docs.rs/{{ project-name }})
[![license](https://img.shields.io/crates/l/{{ project-name }}.svg)](https://crates.io/crates/{{ project-name }})

[![clippy](https://github.com/{{ handle}}/{{ project-name }}/actions/workflows/clippy.yml/badge.svg)](https://github.com/{{handle }}/{{ project-name }}/actions/workflows/clippy.yml)
[![rust](https://github.com/{{ handle }}/{{ project-name }}/actions/workflows/rust.yml/badge.svg)](https://github.com/{{ handle }}/{{ project-name }}/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

{{ description }}

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/{{handle}}/{{ project-name }}.git
cd {{ project-name }}
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.{{ project-name }}]
features = []
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate {{ project-name }};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = {{ project-name }});


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
