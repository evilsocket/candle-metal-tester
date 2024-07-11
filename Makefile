clean:
	cargo clean

build:
	cargo build


build_release:
	cargo build --release

ios_bindings: build
	cargo run --bin uniffi-bindgen generate --library ./target/debug/libwrap.dylib --language swift --out-dir ./libwrap/bindings
 
ios: ios_bindings
	cargo build --release --target=aarch64-apple-ios
	mv ./libwrap/bindings/wrapFFI.modulemap ./libwrap/bindings/module.modulemap
	rm -rf ./Tester/Tester/Wrap.swift
	mv ./libwrap/bindings/wrap.swift ./Tester/Tester/Wrap.swift
	rm -rf "./Tester/Wrap.xcframework"
	xcodebuild -create-xcframework \
        -library ./target/aarch64-apple-ios/release/libwrap.a -headers ./libwrap/bindings \
        -output "./Tester/Wrap.xcframework" > /dev/null
	rm -rf ./libwrap/bindings