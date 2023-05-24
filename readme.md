# Build and test for program compiled natively

### Environment Setup

* Install Rust from https://rustup.rs/

```
$ rustc --version
rustc 1.66.1 (90743e729 2023-01-10) 
$ cargo --version
cargo 1.66.1 (ad779e08b 2023-01-10)  
```

```
$ cargo build
$ cargo test
```

# Build and test for program compiled onchain

### Environment Setup

* Install Solana 1.4.6 from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

```
$ solana --version
solana-cli 1.14.6 (src:cfb2cbe1; feat:2390042548) 
```

```
$ cargo build-sbf
$ cargo test-sbf
```
