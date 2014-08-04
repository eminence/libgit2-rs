all: build
build:
	cargo build
test:
	env RUST_TEST_TASKS=1 cargo test
doc:
	cargo doc
clean:
	cargo clean
commitdoc: clean doc
	git checkout gh-pages && rsync -avP --delete target/doc . && git add doc && git commit -m "Updated docs"
