build:
	podman pull clux/muslrust
	docker run -v $PWD:/volume --rm -t clux/muslrust cargo build
	podman build -t ml.thereisno/quotes . 

debug_run:
	ROCKET_ADDRESS=0.0.0.0 cargo run

clean:
	rm -rf target
	podman system prune --all --force