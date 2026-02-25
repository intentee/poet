.DEFAULT_GOAL := release

RUST_LOG ?= debug

# -----------------------------------------------------------------------------
# Real targets
# -----------------------------------------------------------------------------

package-lock.json: package.json
	npm install --package-lock-only

node_modules: package-lock.json
	npm install --from-lockfile
	touch node_modules

public: node_modules
	./jarmuz-generate.mjs

test_site.AppDir:
	cargo run make app-dir . \
		--name test_site \
		--output-directory . \
		--title "Test Site" \
		--version "1.2.3"

test_site.AppDir/poet: target/debug/poet test_site.AppDir
	cp target/debug/poet test_site.AppDir/poet

test_site-x86_64.AppImage: test_site.AppDir test_site.AppDir/poet
	ARCH=x86_64 appimage-run ~/bin/appimagetool-x86_64.AppImage ./test_site.AppDir

# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	rm -rf node_modules
	rm -rf target

.PHONY: fmt
fmt: node_modules
	./jarmuz-fmt.mjs

.PHONY: watch
watch: node_modules
	./jarmuz-watch.mjs

.PHONY: release
release:
	cargo build --release
