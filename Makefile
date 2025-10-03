SHELL:=bash
DOCKER:=docker
.PHONY=clean docker-build docker-run

clean:
	rm -rf ./target

docker-build:
	mkdir -p ./target/
	cp ~/.config/mobius/config.toml ./target/
	$(DOCKER) build --tag mobius-test:latest .

docker-run: docker-build
	$(DOCKER) run --rm -it mobius-test:latest
