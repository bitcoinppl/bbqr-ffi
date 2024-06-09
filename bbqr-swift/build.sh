#!/bin/bash
# This script builds local bbqrffi Swift language bindings and corresponding bbqrffiFFI.xcframework.
# The results of this script can be used for locally testing your SPM package adding a local package
# to your application pointing at the bbqr-swift directory.

# create all required dirs
mkdir -p bbqrffiFFI.xcframework/ios-arm64/bbqrffiFFI.framework/Headers
mkdir -p bbqrffiFFI.xcframework/ios-arm64_x86_64-simulator/bbqrffiFFI.framework/Headers
mkdir -p bbqrffiFFI.xcframework/macos-arm64_x86_64/bbqrffiFFI.framework/Headers
mkdir -p Sources/Bbqr

rustup default 1.78.0
rustup component add rust-src
rustup target add aarch64-apple-ios      # iOS arm64
rustup target add x86_64-apple-ios       # iOS x86_64
rustup target add aarch64-apple-ios-sim  # simulator mac M1
rustup target add aarch64-apple-darwin   # mac M1
rustup target add x86_64-apple-darwin    # mac x86_64

# go back to ffi root
cd ../bbqr-ffi/ || exit

cargo build --package bbqr-ffi --profile release-smaller --target x86_64-apple-darwin
cargo build --package bbqr-ffi --profile release-smaller --target aarch64-apple-darwin
cargo build --package bbqr-ffi --profile release-smaller --target x86_64-apple-ios
cargo build --package bbqr-ffi --profile release-smaller --target aarch64-apple-ios
cargo build --package bbqr-ffi --profile release-smaller --target aarch64-apple-ios-sim

cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-ios/release-smaller/libbbqrffi.dylib --language swift --out-dir ../bbqr-swift/Sources/Bbqr

mkdir -p target/lipo-ios-sim/release-smaller
lipo target/aarch64-apple-ios-sim/release-smaller/libbbqrffi.a target/x86_64-apple-ios/release-smaller/libbbqrffi.a -create -output target/lipo-ios-sim/release-smaller/libbbqrffi.a

mkdir -p target/lipo-macos/release-smaller
lipo target/aarch64-apple-darwin/release-smaller/libbbqrffi.a target/x86_64-apple-darwin/release-smaller/libbbqrffi.a -create -output target/lipo-macos/release-smaller/libbbqrffi.a

# come back to this dir
cd ../bbqr-swift/ || exit

mv Sources/Bbqr/bbqrffi.swift Sources/Bbqr/Bbqr.swift

cp Sources/Bbqr/bbqrffiFFI.h bbqrffiFFI.xcframework/ios-arm64/bbqrffiFFI.framework/Headers
cp Sources/Bbqr/bbqrffiFFI.h bbqrffiFFI.xcframework/ios-arm64_x86_64-simulator/bbqrffiFFI.framework/Headers
cp Sources/Bbqr/bbqrffiFFI.h bbqrffiFFI.xcframework/macos-arm64_x86_64/bbqrffiFFI.framework/Headers

cp ../bbqr-ffi/target/aarch64-apple-ios/release-smaller/libbbqrffi.a bbqrffiFFI.xcframework/ios-arm64/bbqrffiFFI.framework/bbqrffiFFI
cp ../bbqr-ffi/target/lipo-ios-sim/release-smaller/libbbqrffi.a bbqrffiFFI.xcframework/ios-arm64_x86_64-simulator/bbqrffiFFI.framework/bbqrffiFFI
cp ../bbqr-ffi/target/lipo-macos/release-smaller/libbbqrffi.a bbqrffiFFI.xcframework/macos-arm64_x86_64/bbqrffiFFI.framework/bbqrffiFFI

rm Sources/Bbqr/bbqrffiFFI.h
rm Sources/Bbqr/bbqrffiFFI.modulemap
