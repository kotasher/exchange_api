build:
	podman pull clux/muslrust
	podman run -v .:/volume --rm -t clux/muslrust cargo build --release
	podman build -t ml.thereisno/quotes .

debug_run:
	ROCKET_ADDRESS=0.0.0.0 cargo run

clean:
	rm -rf target
	podman system prune --all --force
