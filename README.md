# Native language bindings for BBQr

<p>
    <a href="https://github.com/bitcoinppl/bbqr-ffi/blob/master/LICENSE"><img alt="Apache-2.0 Licensed" src="https://img.shields.io/badge/Apache--2.0-blue.svg"/></a>
</p>

## Readme

The workspace in this repository creates the `libbbqrffi` multi-language library for the Rust-based
[bbqr] library.

Each supported language and the platform(s) it's packaged for has its own directory. The Rust code in this project is in the `bbqr-ffi` directory and is a wrapper around the [bbqr] library to expose its APIs in a uniform way using the [mozilla/uniffi-rs] bindings generator for each supported target language.

## Supported target languages and platforms

The below directories (a separate repository in the case of bdk-swift) include instructions for using, building, and publishing the native language binding for [bbqr].

| Language | Platform   | Published Package     | Building Documentation | API Docs |
| -------- | ---------- | --------------------- | ---------------------- | -------- |
| Swift    | iOS, macOS | [bbqr-swift (GitHub)] | [Readme bbqr-swift]    |          |
| Kotlin   | Android    | coming soon...        | coming soon...         |          |

## Building and Testing the Libraries

If you are familiar with the build tools for the specific languages you wish to build the libraries for, you can use their normal build/test workflows. We also include some [just](https://just.systems/) files to simplify the work across different languages. If you have the just tool installed on your system, you can simply call the commands defined in the `justfile`s, for example:

```sh
cd bbqr-swift
just build
```

## Minimum Supported Rust Version (MSRV)

This library should compile with any combination of features with Rust 1.78.0.

## Contributing

To add new structs and functions, see the [UniFFI User Guide] and the [uniffi-examples] repository.

## Developing language bindings using uniffi-rs

If you are interested in better understanding the base structure we use here in order to build your own Rust-to-Kotlin/Swift/Python language bindings, check out the [uniffi-bindings-template](https://github.com/thunderbiscuit/uniffi-bindings-template) repository. We maintain it as an example and starting point for other projects that wish to leverage the tech stack used in producing the BDK language bindings.

## Thanks

- This project is made possible thanks to the wonderful work by the [mozilla/uniffi-rs] team.
- Thanks to [SatoshiPortal/bbqr-rust] for the rust implementation
- Thanks to [bdk-ffi], it was used as a template for publishing uniffi bindings

[bbqr]: https://github.com/satoshiportal/bbqr-rust
[bbqr-swift]: https://github.com/bitcoinppl/bbqr-swift
[Kotlin]: https://kotlinlang.org/
[Android Studio]: https://developer.android.com/studio/
[mozilla/uniffi-rs]: https://github.com/mozilla/uniffi-rs
[uniffi-rs]: https://github.com/mozilla/uniffi-rs
[bbqr-swift (Github)]: https://github.com/bitcoinppl/bbqr-swift
[Readme bbqr-swift]: https://github.com/bitcoinppl/bbqr-swift
[SatoshiPortal/bbqr-rust]: https://github.com/satoshiportal/bbqr-rust
[UniFFI User Guide]: https://mozilla.github.io/uniffi-rs/
[uniffi-examples]: https://thunderbiscuit.github.io/uniffi-examples/
[bdk-ffi]: https://github.com/bitcoindevkit/bdk-ffi
