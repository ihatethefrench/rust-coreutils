debug:
	cargo build
release:
	RUSTFLAGS="" cargo build --release
	cd target/release && strip -s rm cat clear sh
clean:
	rm -rf target/
