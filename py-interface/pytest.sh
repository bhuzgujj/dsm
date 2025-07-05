# /bin/bash

python -m venv .venv
./.venv/Scripts/activate
pip install -r requirements.txt

maturin develop

./.venv/Scripts/python interface.py