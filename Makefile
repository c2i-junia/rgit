all: build

build:
	cargo build
	cp target/debug/rgit .

clean:
	cargo clean
	rm -f rgit
