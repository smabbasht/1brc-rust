# 1 Billion Row Challenge

This repository is the implementation of popular 1 billion row challenge in rust, known as `1brc`.

## Reproduction of results
In order to get started you need `rust` and `python` installed on your system where `python` is only used for generating data.

```
git clone https://github.com/smabbasht/1brc-rust
cd 1brc-rust/data
python3 createMeasurements.py 1000000000
cd ..
cargo run
```
