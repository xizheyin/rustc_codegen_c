# Working with `rustc`

Kani is developed on the top of the Rust compiler, which is not distributed on [crates.io](https://crates.io/) and depends on
bootstrapping mechanisms to properly build its components.
Thus, our dependency on `rustc` crates are not declared in our `Cargo.toml`.

Below are a few hacks that will make it easier to develop on the top of `rustc`.

## Code analysis for `rustc` definitions

IDEs rely on `cargo` to find dependencies and sources to provide proper code analysis and code completion.
In order to get these features working for `rustc` crates, you can do the following:

### VSCode

Add the following to the `rust-analyzer` extension settings in `settings.json`:
```json
    "rust-analyzer.rustc.source": "discover",
    "rust-analyzer.workspace.symbol.search.scope": "workspace_and_dependencies",
```

Ensure that any packages that use `rustc` data structures have the following line set in their `Cargo.toml`

```toml
[package.metadata.rust-analyzer]
rustc_private=true
```

You may also need to install the `rustc-dev` package using rustup

```
rustup toolchain install nightly --component rustc-dev
```