rootProject.name = "bbqr-android"

include(":lib")
includeBuild("plugins")

pluginManagement {
    repositories {
        gradlePluginPortal()
        google()
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
        google()
    }
}
