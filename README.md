# CasperLabs Smart Contract Examples

Each subdirectory contains an example of a smart contract definition and a companion contract that calls it.


## Building

To build the contracts you'll need to install [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To build all the contracts:

```
cargo build --release
```

To build a specific contract and its companion:

```
cargo build --release -p store-hello-name
cargo build --release -p call-hello-name
```

After building a contract, you will find the corresponding wasm file in `target/wasm32-unknown-unknown/release`.

**NOTE**: The `--release` flag is currently necessary in order to build optimized wasm files that can be deployed from a CasperLabs Node.

## Using

To deploy a compiled contract to a CasperLabs node, please see the CasperLabs [Developer Documentation](https://github.com/CasperLabs/CasperLabs/blob/dev/DEVELOPER.md#deploying-data).
