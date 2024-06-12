plugins {
    id("com.android.library").version("8.3.1").apply(false)
    id("org.jetbrains.kotlin.android").version("1.9.23").apply(false)
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.bitcoinppl.bbqr.plugins.generate-android-bindings").apply(false)
    id("com.gradleup.nmcp").version("0.0.8")

}

// library version is defined in gradle.properties
val libraryVersion: String by project

val sonatypeUsername: String? by project // this is defined in ~/.gradle/gradle.properties
val sonatypePassword: String? by project // this is defined in ~/.gradle/gradle.properties

nmcp {
  publishAllProjectsProbablyBreakingProjectIsolation {
    username = sonatypeUsername ?: System.getenv("SONATYPE_USERNAME")
    password = sonatypePassword ?: System.getenv("SONATYPE_PASSWORD")
    // publicationType = "AUTOMATIC"
    publicationType = "USER_MANAGED"
  }
}

