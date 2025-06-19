build-docker:
	mkdir -p ./target/
	cp ~/.config/mobius/config.toml ./target/
	container build --tag mobius-test