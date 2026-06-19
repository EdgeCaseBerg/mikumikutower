

all: fmt wasm native

fmt:
	cargo fmt

wasm:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/mikumikutower.wasm --out-dir web/pkg --target web
	cp -r assets web/assets
	cp index.html web/index.html

native:
	cargo build

clean:
	rm -rf target/ web/

doc:
	cargo doc
	cargo doc --target wasm32-unknown-unknown	