plugins {
    id("com.android.library").version("8.3.1").apply(false)
    id("org.jetbrains.kotlin.android").version("1.9.23").apply(false)
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.bitcoinppl.bbqr.plugins.generate-android-bindings").apply(false)
    id("io.github.gradle-nexus.publish-plugin").version("1.1.0").apply(true)
}

// library version is defined in gradle.properties
val libraryVersion: String by project

