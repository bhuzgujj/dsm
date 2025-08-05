# Python version of DSM
This is a python usable dsm.

## Setup
Requires:
- [Python v3.12.5](https://www.python.org/downloads/release/python-3125/)
- [Rust](https://www.rust-lang.org/)

Setup venv
```
python -m venv .venv
./.venv/Scripts/activate
pip install -r requirements.txt
```

## Create python package
```sh
# Generate python interfaces for intellisense
cargo run --bin stub_gen 

# Generate development python package
maturin develop

# Generate python package
maturin build
```