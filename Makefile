.SHELL := /usr/local/bin/bash
debug:
	cargo build
release:
	cargo build --release
	cd target/release && strip -s rm cat clear
clean:
	rm -rf target/
