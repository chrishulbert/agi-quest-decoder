help:
	cat Makefile

run:
	RUST_BACKTRACE=1 cargo run data/sq2

clean:
	rm -f Output*

test:
	cargo test

shrink: # Ignore error 98 means file(s) was/were already shrunk.
	-pngquant --force --skip-if-larger --ext .png Output*.png
