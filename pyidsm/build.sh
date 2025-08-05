# /bin/bash

python -m venv .venv
./.venv/Scripts/activate
./.venv/Scripts/pip install -r requirements.txt

cargo run --bin stub_gen
maturin build