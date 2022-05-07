#!/bin/bash
cargo build --release
mkdir bin
mv ./target/release/bootstrap ./bin
mv ./target/release/encryption-cli ./bin
mv ./target/release/hashtester ./bin
echo "files ready in ./bin"
