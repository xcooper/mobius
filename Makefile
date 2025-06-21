build-docker:
	mkdir -p ./target/
	cp ~/.config/mobius/config.toml ./target/
	podman build --tag mobius-test --file ./Dockerfile