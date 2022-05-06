#### what is this?

This is a small program to test the speed of sha3 hashing algorithms to store passwords

#### usage

```
cargo run --release NUM         # NUM being the number of batches (default 5)
cargo run --release --help      # displays small help menu
cargo run --release --details   # shows the speed of generating each hash in ns (not recommended, spams terminal)
```

each 'batch' creates 1000000 hashes
