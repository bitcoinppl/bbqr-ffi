plugins {
    id("java-gradle-plugin")
    `kotlin-dsl`
}

gradlePlugin {
    plugins {
        create("uniFfiAndroidBindings") {
            id = "org.bitcoinppl.plugins.generate-android-bindings"
            implementationClass = "org.bitcoinppl.plugins.UniFfiAndroidPlugin"
        }
    }
}
