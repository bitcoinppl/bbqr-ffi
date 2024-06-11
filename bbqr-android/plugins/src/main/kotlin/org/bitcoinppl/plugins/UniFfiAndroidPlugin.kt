package org.bitcoinppl.plugins

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.tasks.Copy
import org.gradle.api.tasks.Exec
import org.gradle.kotlin.dsl.environment
import org.gradle.kotlin.dsl.getValue
import org.gradle.kotlin.dsl.provideDelegate
import org.gradle.kotlin.dsl.register

internal class UniFfiAndroidPlugin : Plugin<Project> {
    override fun apply(target: Project): Unit = target.run {
        val llvmArchPath = when (operatingSystem) {
            OS.MAC -> "darwin-x86_64"
            OS.LINUX -> "linux-x86_64"
            OS.OTHER -> throw Error("Cannot build Android library from current architecture")
        }

        // arm64-v8a is the most popular hardware architecture for Android
        val buildAndroidAarch64Binary by tasks.register<Exec>("buildAndroidAarch64Binary") {

            workingDir("${projectDir}/../../bbqr-ffi")
            val cargoArgs: List<String> = listOf("ndk", "--target", "aarch64-linux-android", "build", "--package", "bbqr-ffi", "--profile", "release-smaller")

            executable("cargo")
            args(cargoArgs)

            environment(
                // add build toolchain to PATH
                Pair("CFLAGS", "-D__ANDROID_MIN_SDK_VERSION__=21"),
            )
            doLast {
                println("Native library for bbqr-android on aarch64 built successfully")
            }
        }

        // the x86_64 version of the library is mostly used by emulators
        val buildAndroidX86_64Binary by tasks.register<Exec>("buildAndroidX86_64Binary") {

            workingDir("${project.projectDir}/../../bbqr-ffi")
            val cargoArgs: List<String> = listOf("ndk", "--target", "x86_64-linux-android", "build", "--package", "bbqr-ffi", "--profile", "release-smaller")

            executable("cargo")
            args(cargoArgs)

            environment(
                // add build toolchain to PATH
                Pair("CFLAGS", "-D__ANDROID_MIN_SDK_VERSION__=21"),
            )

            doLast {
                println("Native library for bbqr-android on x86_64 built successfully")
            }
        }

        // armeabi-v7a version of the library for older 32-bit Android hardware
        val buildAndroidArmv7Binary by tasks.register<Exec>("buildAndroidArmv7Binary") {

            workingDir("${project.projectDir}/../../bbqr-ffi")
            val cargoArgs: List<String> = listOf("ndk", "--target", "armv7-linux-androideabi", "build", "--package", "bbqr-ffi", "--profile", "release-smaller")

            executable("cargo")
            args(cargoArgs)

            environment(
                // add build toolchain to PATH
                Pair("CFLAGS", "-D__ANDROID_MIN_SDK_VERSION__=21"),
            )

            doLast {
                println("Native library for bbqr-android on armv7 built successfully")
            }
        }

        // move the native libs build by cargo from target/<architecture>/release/
        // to their place in the bbqr-android library
        // the task only copies the available binaries built using the buildAndroid<architecture>Binary tasks
        val moveNativeAndroidLibs by tasks.register<Copy>("moveNativeAndroidLibs") {

            dependsOn(buildAndroidAarch64Binary)
            dependsOn(buildAndroidArmv7Binary)
            dependsOn(buildAndroidX86_64Binary)

            into("${project.projectDir}/../lib/src/main/jniLibs/")

            into("arm64-v8a") {
                from("${project.projectDir}/../../bbqr-ffi/target/aarch64-linux-android/release-smaller/libbbqrffi.so")
            }

            into("x86_64") {
                from("${project.projectDir}/../../bbqr-ffi/target/x86_64-linux-android/release-smaller/libbbqrffi.so")
            }

            into("armeabi-v7a") {
                from("${project.projectDir}/../../bbqr-ffi/target/armv7-linux-androideabi/release-smaller/libbbqrffi.so")
            }

            doLast {
                println("Native binaries for Android moved to ./lib/src/main/jniLibs/")
            }
        }

        // generate the bindings using the bbqr-ffi-bindgen tool located in the bbqr-ffi submodule
        val generateAndroidBindings by tasks.register<Exec>("generateAndroidBindings") {
            dependsOn(moveNativeAndroidLibs)

            val libraryPath = "${project.projectDir}/../../bbqr-ffi/target/aarch64-linux-android/release-smaller/libbbqrffi.so"
            workingDir("${project.projectDir}/../../bbqr-ffi")
            val cargoArgs: List<String> = listOf("run", "--bin", "uniffi-bindgen", "generate", "--library", libraryPath, "--language", "kotlin", "--out-dir", "../bbqr-android/lib/src/main/kotlin", "--no-format")

            executable("cargo")
            args(cargoArgs)

            doLast {
                println("Android bindings file successfully created")
            }
        }

        // create an aggregate task which will run the required tasks to build the Android libs in order
        // the task will also appear in the printout of the ./gradlew tasks task with group and description
        tasks.register("buildAndroidLib") {
            group = "bitcoinppl"
            description = "Aggregate task to build Android library"

            dependsOn(
                buildAndroidAarch64Binary,
                buildAndroidX86_64Binary,
                buildAndroidArmv7Binary,
                moveNativeAndroidLibs,
                generateAndroidBindings
            )
        }
    }
}
