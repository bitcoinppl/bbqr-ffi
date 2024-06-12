# bbqr-android

This project builds an .aar package for the Android platform that provide Kotlin language bindings for the [`bbqr`] library. The Kotlin language bindings are created by the [`bbqr-ffi`] project which is included in the root of this repository.

## How to Use

To use the Kotlin language bindings for [`bbqr`] in your Android project add the following to your gradle dependencies:

```kotlin
repositories {
    mavenCentral()
}

dependencies {
    implementation("org.bitcoinppl:bbqr-android:0.3.1")
}
```

### Example Projects

- [bbqr-kotlin-example-wallet](https://github.com/bitcoindevkit/bbqr-kotlin-example-wallet)
- [Devkit Wallet](https://github.com/thunderbiscuit/devkit-wallet)
- [Padawan Wallet](https://github.com/thunderbiscuit/padawan-wallet)

### How to build

_Note that Kotlin version `1.9.23` or later is required to build the library._

1. Clone this repository.

```shell
git clone https://github.com/bitcoinppl/bbqr-ffi
```

2. Install Rust (note that we are currently building using Rust 1.77.1):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default 1.78.0
```

3. Install required targets

```sh
rustup target add x86_64-linux-android aarch64-linux-android armv7-linux-androideabi
```

4. Install Android SDK and Build-Tools for API level 30+
5. Setup `ANDROID_SDK_ROOT`

```shell
# macOS
export ANDROID_SDK_ROOT=~/Library/Android/sdk

# linux
export ANDROID_SDK_ROOT=/usr/local/lib/android/sdk
```

6. Build kotlin bindings

```sh
# build Android library
cd bbqr-android
./gradlew buildAndroidLib
```

## How to publish to your local Maven repo

Install the required JDK version 17.

```shell
brew install openjdk@17
```

Run the gradlew command to publish to local

```shell
cd bbqr-android
./gradlew publishToMavenLocal -P localBuild
```

Note that the commands assume you don't need the local libraries to be signed. If you do wish to sign them, simply set your `~/.gradle/gradle.properties` signing key values like so:

```properties
signing.gnupg.keyName=<YOUR_GNUPG_ID>
signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
```

and use the `publishToMavenLocal` task without the `localBuild` flag:

```shell
./gradlew publishToMavenLocal
```

## Known issues

### JNA dependency

Depending on the JVM version you use, you might not have the JNA dependency on your classpath. The exception thrown will be

```shell
class file for com.sun.jna.Pointer not found
```

The solution is to add JNA as a dependency like so:

```kotlin
dependencies {
    // ...
    implementation("net.java.dev.jna:jna:5.12.1")
}
```

### x86 emulators

For some older versions of macOS, Android Studio will recommend users install the x86 version of the emulator by default. This will not work with the bbqr-android library, as we do not support 32-bit architectures. Make sure you install an x86_64 emulator to work with bbqr-android.

[`bbqr`]: https://github.com/satoshiportal/bbqr-rust
[`bbqr-ffi`]: https://github.com/bitcoinppl/bbqr-ffi
