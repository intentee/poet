.DEFAULT_GOAL := target/release/poet

RUST_LOG ?= debug

POET_SOURCES := $(shell find poet/src poet_mcp/src rhai_components/src -name '*.rs') Cargo.toml Cargo.lock poet/Cargo.toml poet_mcp/Cargo.toml rhai_components/Cargo.toml

# -----------------------------------------------------------------------------
# Real targets
# -----------------------------------------------------------------------------

node_modules: package-lock.json
	npm ci
	touch node_modules

package-lock.json: package.json
	npm install --package-lock-only

target/debug/poet: $(POET_SOURCES)
	cargo build

target/release/poet: $(POET_SOURCES)
	cargo build --release

test_site-x86_64.AppImage: test_site.AppDir test_site.AppDir/poet
	ARCH=x86_64 appimage-run ~/bin/appimagetool-x86_64.AppImage ./test_site.AppDir

test_site.AppDir:
	cargo run make app-dir . \
		--name test_site \
		--output-directory . \
		--title "Test Site" \
		--version "1.2.3"

test_site.AppDir/poet: target/debug/poet test_site.AppDir
	cp target/debug/poet test_site.AppDir/poet

# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	rm -rf node_modules
	rm -rf target

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: coverage
coverage: node_modules
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest --workspace --no-report
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated poet=80 \
		--gated poet_mcp=100 \
		--gated rhai_components=100

.PHONY: coverage-clean
coverage-clean:
	cargo llvm-cov clean --workspace
	rm -f target/llvm-cov.json

.PHONY: coverage-report
coverage-report:
	cargo llvm-cov nextest --workspace --html

.PHONY: fmt
fmt: node_modules
	./jarmuz-fmt.mjs

.PHONY: test
test:
	cargo test --workspace

.PHONY: watch
watch: node_modules
	./jarmuz-watch.mjs
