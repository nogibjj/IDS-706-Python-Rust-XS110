# Name of your rust binary after compilation
RUST_TARGET=./target/release/main

install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

# Compile Rust Code
$(RUST_TARGET): main.rs
	@echo "Compiling Rust code..."
	@cargo build --release

# Install Rust dependencies (assuming you have a Cargo.toml)
rust-install:
	cargo install --path .

format:
	black *.py

# Format Rust code
rust-format:
	cargo fmt

lint:
	#pylint --disable=R,C --ignore-patterns=test_.*?py *.py
	ruff check *.py

# Lint Rust code
rust-lint:
	cargo clippy

test:
	python -m pytest -vv --cov=main test_*.py
	pytest --nbval test.ipynb

# Test Rust code 
rust-test:
	cargo test

# Run Rust code
run-rust: 
	@echo "Running Rust code..."
	@bash -c "time cargo run"
	

# Run Python code
run-python:
	@echo "Running Python code..."
	@bash -c "time python3 main.py"

# Compare resource usage of Python and Rust code
compare: run-python run-rust

deploy:
	#deploy goes here

all: install lint deploy format test rust-install rust-lint rust-format rust-test

# Clean compiled Rust files
clean:
	cargo clean

.PHONY: install format lint test deploy compare run-python run-rust rust-install rust-format rust-lint rust-test clean
