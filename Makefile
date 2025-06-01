# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
REPO = $(shell echo ${DIR} | sed 's/.*\///' | tr '[:upper:]' '[:lower:]')

# ****** Rust Constants ******
CARGO = /root/.cargo/bin/cargo
CODE_VOLUME = -v ${DIR}:/${REPO}
CARGO_REGISTRY = -v cargo_registy:/root/.cargo/registry

# ****** Docker Constants ******
DOCKER_RUN = docker run --rm -t
DOCKER_RUN_IT = ${DOCKER_RUN} -it --name ${REPO}

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} -w /${REPO}

terminal:
	${DOCKER_RUN_IT}_terminal ${RUN_ATTRS} jkutkut/docker4rust

reset_file_permissions:
	sudo chown -R ${USER}:${USER} .

test:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust test --all-features

test_backtrace:
	${DOCKER_RUN} ${RUN_ATTRS} -e RUST_BACKTRACE=1 --entrypoint cargo jkutkut/docker4rust test --all-features

test_watch:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust watch --clear test --all-features

test_watch_dumper:
	${DOCKER_RUN} -it --name ${REPO}_dumper ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust watch --clear test --features dumper

test_watch_debug:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint cargo -e RUST_BACKTRACE=1 jkutkut/docker4rust watch --clear test --all-features

doc:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust doc --lib --examples --document-private-items

doc_watch:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust watch --clear -x test -x "doc --lib --examples --document-private-items"

doc_release:
	@echo "Ensuring repo has no uncommited changes..."
	@git diff --quiet && git diff --cached --quiet || (echo "Error: Repository not clean" && false)
	@echo "${REPO} is clean."
	@echo "Generating docs..."
	make doc
	sudo chown -R ${USER}:${USER} target
	@echo "Preparing for commit..."
	rm -rf /tmp/osmia-doc
	cp -r target/doc /tmp/osmia-doc
	echo "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)" > /tmp/osmia-version.txt
	@echo "Committing docs..."
	git checkout documentation
	rm -rf ./*
	cp -r /tmp/osmia-doc/* .
	git add .
	cat /tmp/osmia-version.txt | git commit -F -
	@echo "Cleaning up..."
	rm -rf /tmp/osmia-doc
	rm -rf /tmp/osmia-version.txt
	@echo "Done! Publishing docs..."
	@git push
	@git checkout main

stop:
	docker rm -f ${REPO}
	docker rm -f ${REPO}_dumper

clean:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust clean

# ****** Logo ******
LOGO_PATH = res
LOGO_FULL_PATH = ${LOGO_PATH}/logo.svg
LOGO_GENERATOR_SRC = src/utils/logo-generator.rs
LOGO_GENERATOR_TARGET = ./target/logo-generator

logo: ${LOGO_FULL_PATH}

logo_watch:
	@# Needs inotify-tools
	@while inotifywait -e modify ${LOGO_GENERATOR_SRC} > /dev/null || true; do \
		make logo; \
	done

${LOGO_FULL_PATH}: ${LOGO_GENERATOR_TARGET}
	$< $@

${LOGO_GENERATOR_TARGET}: ${LOGO_GENERATOR_SRC}
	${DOCKER_RUN_IT} ${CODE_VOLUME} -w /${REPO} --entrypoint rustc jkutkut/docker4rust $< -o $@

# ****** Git ******

prepare_commit: hooks
	${EDITOR} Cargo.toml
	make test
	git add Cargo.toml Cargo.lock; git add -N .;
	git add -p

hooks:
	git config core.hooksPath .githooks
	# git config --unset core.hooksPath
