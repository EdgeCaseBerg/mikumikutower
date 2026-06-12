

all: fmt wasm native

fmt:
	cargo fmt

wasm:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/mikumikutower.wasm --out-dir web/pkg --target web
	cp -r assets web/pkg/assets

native:
	cargo build

clean:
	rm -rf target/ web/