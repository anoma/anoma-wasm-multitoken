cargo = $(env) cargo

debug:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'test_runners' \
			--target wasm32-unknown-unknown \
			--target-dir build/wasm32 \
			--out-dir build/debug/

release:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'test_runners' \
			--release \
			--target wasm32-unknown-unknown \
			--target-dir build/wasm32 \
			--out-dir build/release/

test-runners:
	$(cargo) -Z unstable-options \
		build \
			--target x86_64-unknown-linux-gnu \
			--target-dir build/x86_64-unknown-linux-gnu \
			--package 'test_runners' \
			--out-dir build/test

.PHONY : debug release
