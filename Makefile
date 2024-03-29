PROJECT="rust-wasm-devenv"

# Sets up a container in which you can build and run the project keeping your local env clean.
setup-env: docker-build docker-run

docker-build:
	docker build -t $(PROJECT) .
docker-run:
	docker run -it -p 8585:8585 --rm -v $(shell pwd):/data -w /data $(PROJECT) /bin/bash

clean:
	-rm -rf dist pkg target node_modules