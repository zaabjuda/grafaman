.PHONY: install build

install:
	cp target/release/grafaman /usr/local/bin/grafaman

build:
	cargo build --release

output_release:
	cp target/release/grafaman ../build/grafaman

make_docker:
	docker build --force-rm -t grafaman .
