rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

build:
    cd rust-cli-binary && cargo build --quiet

format:
	cd rust-cli-binary && cargo fmt --quiet

lint:
	cd rust-cli-binary && cargo clippy --quiet

clean:
	cd rust-cli-binary && cargo clean

run:
	cd rust-cli-binary && cargo run

test:
	cd rust-cli-binary && cargo test --quiet

run:
	cargo run

release:
	cargo build --release