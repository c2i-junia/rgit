all: build

build:
	cargo build
	cp target/debug/rgit .

test:
	cargo test -- --test-threads=1 

clean:
	cargo clean
	rm -f rgit
