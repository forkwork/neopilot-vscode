# The Neopilot Rust Extension API

This crate lets you write extensions for Neopilot in Rust.

## Extension Manifest

You'll need an `extension.toml` file at the root of your extension directory, with the following structure:

```toml
id = "my-extension"
name = "My Extension"
description = "..."
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/your/extension-repository"
```

## Cargo metadata

Neopilot extensions are packaged as WebAssembly files. In your Cargo.toml, you'll
need to set your `crate-type` accordingly:

```toml
[dependencies]
neopilot_extension_api = "0.6.0"

[lib]
crate-type = ["cdylib"]
```

## Implementing an Extension

To define your extension, create a type that implements the `Extension` trait, and register it.

```rust
use neopilot_extension_api as neopilot;

struct MyExtension {
    // ... state
}

impl neopilot::Extension for MyExtension {
    // ...
}

neopilot::register_extension!(MyExtension);
```

## Testing your extension

To run your extension in Neopilot as you're developing it:

- Make sure you have [Rust installed](https://www.rust-lang.org/learn/get-started)
- Have the `wasm32-wasip2` target installed (`rustup target add wasm32-wasip2`)
- Open the extensions view using the `neopilot: extensions` action in the command palette.
- Click the `Install Dev Extension` button in the top right
- Choose the path to your extension directory.

## Compatible Neopilot versions

Extensions created using newer versions of the Neopilot extension API won't be compatible with older versions of Neopilot.

Here is the compatibility of the `neopilot_extension_api` with versions of Neopilot:

| Neopilot version | `neopilot_extension_api` version |
| ----------- | --------------------------- |
| `0.186.x`   | `0.0.1` - `0.5.0`           |
| `0.184.x`   | `0.0.1` - `0.4.0`           |
| `0.178.x`   | `0.0.1` - `0.3.0`           |
| `0.162.x`   | `0.0.1` - `0.2.0`           |
| `0.149.x`   | `0.0.1` - `0.1.0`           |
| `0.131.x`   | `0.0.1` - `0.0.6`           |
| `0.130.x`   | `0.0.1` - `0.0.5`           |
| `0.129.x`   | `0.0.1` - `0.0.4`           |
| `0.128.x`   | `0.0.1`                     |
