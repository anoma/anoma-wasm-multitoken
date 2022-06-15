cargo = $(env) cargo

debug:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--target wasm32-unknown-unknown \
			--target-dir build/wasm32 \
			--out-dir build/debug/

release:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--release \
			--target wasm32-unknown-unknown \
			--target-dir build/wasm32 \
			--out-dir build/release/

.PHONY : debug release
