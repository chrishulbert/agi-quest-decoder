help:
	cat Makefile

run:
	RUST_BACKTRACE=1 cargo run data/sqtlc

run-sq2:
	RUST_BACKTRACE=1 cargo run data/sq2

run-lsl1:
	RUST_BACKTRACE=1 cargo run data/lsl1

build:
	cargo build

clean:
	rm -f Output*

test:
	cargo test

compress-animation: *.animation.png
	for f in *.animation.png; do \
		apngasm --force -o "$$f" "$$f"; \
	done

compress-static: *.static.png
	for f in *.static.png; do \
		pngquant --force --skip-if-larger --output "$$f" "$$f"; \
	done

compress: compress-static compress-animation
