.DEFAULT_GOAL := help

VERSION_CRATES := cenyslovensko_api/Cargo.toml \
                  cenyslovensko_vendor_api/Cargo.toml \
                  cenyslovensko_version_api/Cargo.toml \
                  cenyslovensko_web_client/Cargo.toml \
                  cenyslovensko_rpc_server/Cargo.toml

# Read current version from umbrella crate
CURRENT_VERSION := $(shell grep -m1 '^version' cenyslovensko_api/Cargo.toml | sed 's/version = "\(.*\)"/\1/')

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: build
build: ## Build entire workspace (debug)
	cargo build --workspace

.PHONY: build-release
build-release: ## Build entire workspace (release)
	cargo build --workspace --release

.PHONY: build-rpc-server
build-rpc-server: ## Build only the RPC server binary
	cargo build -p cenyslovensko_rpc_server

.PHONY: test
test: ## Run all Rust tests
	cargo test --workspace

.PHONY: test-python
test-python: ## Run Python binding tests
	cd bindings/python && python -m pytest

.PHONY: test-ruby
test-ruby: ## Run Ruby binding tests
	cd bindings/ruby && ruby -Ilib -Itest test/test_all.rb

.PHONY: test-nodejs
test-nodejs: ## Run Node.js binding tests
	cd bindings/nodejs && node --test

.PHONY: test-all
test-all: test test-python test-ruby test-nodejs ## Run all tests (Rust + bindings)

.PHONY: docs
docs: docs-rust ## Generate all documentation

.PHONY: docs-rust
docs-rust: ## Generate Rust API documentation
	cargo doc --workspace --no-deps

.PHONY: docs-python
docs-python: ## Generate Python documentation
	cd bindings/python && pdoc cenyslovensko_bindings -o ../../target/docs/python

.PHONY: docs-ruby
docs-ruby: ## Generate Ruby documentation
	rm -rf target/docs/ruby && rdoc bindings/ruby/lib --output target/docs/ruby

.PHONY: docs-nodejs
docs-nodejs: ## Generate Node.js documentation
	cd bindings/nodejs && npx jsdoc src -d ../../target/docs/nodejs

.PHONY: fmt
fmt: ## Format all Rust source files
	cargo fmt --all

.PHONY: lint
lint: ## Run Clippy over entire workspace
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: version
version: ## Print current crate version
	@echo $(CURRENT_VERSION)

.PHONY: tag
tag: ## Create and push a release tag (usage: make tag VERSION=x.y.z)
ifndef VERSION
	$(error VERSION is required - run: make tag VERSION=x.y.z)
endif
	@echo "Bumping version from $(CURRENT_VERSION) → $(VERSION)"
	@for f in $(VERSION_CRATES); do \
	  sed -i.bak "s/^version = \"$(CURRENT_VERSION)\"/version = \"$(VERSION)\"/" $$f && rm $$f.bak; \
	done
	@# Update cross-crate version references inside Cargo.toml files
	@for f in $(VERSION_CRATES); do \
	  sed -i.bak "s/version = \"$(CURRENT_VERSION)\"/version = \"$(VERSION)\"/g" $$f && rm $$f.bak; \
	done
	cargo check --workspace
	git add $(VERSION_CRATES)
	git commit -m "chore: release v$(VERSION)"
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	git push origin HEAD "v$(VERSION)"
	@echo "Tagged and pushed v$(VERSION)"

.PHONY: clean
clean: ## Remove build artifacts
	cargo clean
