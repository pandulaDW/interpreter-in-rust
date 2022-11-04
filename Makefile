build:
	cargo build --release
	sudo cp ./target/release/interpreter /usr/local/bin
	sudo mv /usr/local/bin/interpreter /usr/local/bin/monkey