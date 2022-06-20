cargo = $(env) cargo

debug:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'test_runners' \
			--target wasm32-unknown-unknown \
			--target-dir build/cache/wasm32-unknown-unknown \
			--out-dir build/debug/

release:
	$(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'test_runners' \
			--release \
			--target wasm32-unknown-unknown \
			--target-dir build/cache/wasm32-unknown-unknown \
			--out-dir build/release/

test-runners:
	$(cargo) -Z unstable-options \
		build \
			--target x86_64-unknown-linux-musl \
			--target-dir build/cache/x86_64-unknown-linux-musl \
			--package 'test_runners' \
			--out-dir build/tests

docker: debug test-runners
	docker compose build

.PHONY : debug release docker
