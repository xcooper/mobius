build-docker:
	mkdir -p ./target/
	cp ~/.config/mobius/config.toml ./target/
	docker build --tag mobius-test .
