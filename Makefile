SHELL:=bash
DOCKER:=docker
.PHONY=build docker-build docker-run

build:
	cargo build

docker-build:
	mkdir -p ./target/
	cp ~/.config/mobius/config.toml ./target/
	$(DOCKER) build --tag mobius-test:latest .

docker-run: docker-build
	$(DOCKER) run --rm -it mobius-test:latest
