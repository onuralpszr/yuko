set -euxo pipefail

sudo apt-get update
sudo apt-get install -y python3-dev python3-pip python3-venv libclang-dev bat exa
sudo python3 -m pip install cffi virtualenv pipx

pipx ensurepath
pipx install uniffi-bindgen
pipx install cargo-deny
pipx install pre-commit

rustup update
curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
