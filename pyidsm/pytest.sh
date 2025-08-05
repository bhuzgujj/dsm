# /bin/bash

python -m venv .venv
./.venv/Scripts/activate
./.venv/Scripts/pip install -r requirements.txt

maturin develop

./.venv/Scripts/python interface.py