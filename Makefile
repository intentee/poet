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

# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	rm -rf node_modules
	rm -rf target

.PHONY: fmt
fmt: prettier
	cargo fmt

.PHONY: prettier
prettier: node_modules
	# npm exec prettier -- \
	# 	--plugin=prettier-plugin-organize-imports \
	# 	--write \
	# 	jarmuz \
	# 	resources \
	# 	*.mjs \
