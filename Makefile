rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

release:
	cd rust-cli-binary && cargo build --release

build:
	cd rust-cli-binary && cargo build --quiet

format:
	cd rust-cli-binary && cargo fmt --quiet

lint:
	cd rust-cli-binary && cargo clippy --quiet

run:
	cd rust-cli-binary && cargo run

test:
	cd rust-cli-binary && cargo test --quiet

clean:
	cd rust-cli-binary && cargo clean