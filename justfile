############################################
# Just commands for Kellnr
# 
# It's recommended to use the just commands
# instead of the cargo commands, as they
# provide additional functionality.
############################################


##########################################
# Common commands
##########################################

check:
	cargo check

build:
	cargo build

build-release:
	cargo build --release

run: npm-build build
	cargo run

clean:
	cargo clean

test: # Run all tests except the Postgresql integration tests, which require Docker
	cargo nextest run --workspace -E 'not binary_id(db::postgres_test)'

test-all:
	{{test_all}}

clean-node:
	rm -rf ui/node_modules
	rm -rf ui/package-lock.json

clean-all: clean clean-node

npm-build: npm-install
	cd ui && npm run build
	mkdir -p static
	cp -r ui/dist/* static/

npm-install:
	cd ui && npm install

##########################################
# Commands used by the Nix package manager
##########################################

# Used to create the needed Nix expressions for the Node.js dependencies.
# Run this command everytime you edit the ui/package.json file.
node2nix: clean-node patch-package
	node2nix --development \
		--input ui/nix/package.json \
		--node-env ui/nix/node-env.nix \
		--composition ui/nix/default.nix \
		--output ui/nix/node-package.nix

patch-package: 
	jd -o ui/nix/package.json \
	-p \
	-f patch ui/nix/package-patch.json ui/package.json || true 


##########################################
# Commands used by the Github Actions CI
##########################################

# Set the target for the ci-release command.
# The target can be "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", 
# or "armv7-unknown-linux-gnueabihf".
# It's used by the Github Actions CI to build the release binary for the specified target.
target := "x86_64-unknown-linux-gnu"

ci-test: npm-build
	cargo test --workspace --profile ci-dev

ci-release: npm-build
        cargo build --profile ci-release --target {{target}} --features vendored-openssl


##########################################
# Aliases
##########################################

alias b := build
alias br := build-release
alias r := run
alias t := test
alias ta := test-all
alias c := check

# "true" if docker is installed, "false" otherwise
# Docker is needed for the Postgresql integration tests
has_docker := if `command -v docker > /dev/null 2>&1; echo $?` == "0" { "true" } else { "false" }
test_all := if has_docker == "true" { "cargo nextest run --workspace" } else { "echo 'ERROR: Docker is not installed. The Postgresql integration tests require Docker'" }

docker:
	echo "{{has_docker}}"
