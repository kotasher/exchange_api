build:
	# cargo build --release
	cargo build --release --target=x86_64-unknown-linux-musl
	podman build -t ml.thereisno/quotes . 

clean:
	rm -rf target
	podman system prune --all --force