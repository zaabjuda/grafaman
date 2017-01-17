.PHONY: install build

install:
	cp target/release/grafaman /usr/local/bin/grafaman

build:
	cargo build --release
