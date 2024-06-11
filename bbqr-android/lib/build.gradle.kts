import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

// library version is defined in gradle.properties
val libraryVersion: String by project

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.gradle.maven-publish")
    id("org.gradle.signing")

    // Custom plugin to generate the native libs and bindings file
    id("org.bitcoinppl.bbqr.plugins.generate-android-bindings")
}

android {
    namespace = "org.bitcoinppl.bbqr"
    compileSdk = 34

    defaultConfig {
        minSdk =  22
        targetSdk = 34
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }

    publishing {
        singleVariant("release") {
            withSourcesJar()
            withJavadocJar()
        }
    }
}

kotlin {
    tasks.withType<KotlinCompile>().configureEach {
        kotlinOptions {
            jvmTarget = "17"
        }
    }
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.4.0")
    implementation("androidx.core:core-ktx:1.7.0")
    api("org.slf4j:slf4j-api:1.7.30")

    androidTestImplementation("com.github.tony19:logback-android:2.0.0")
    androidTestImplementation("androidx.test.ext:junit:1.1.3")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test:1.6.10")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test-junit:1.6.10")
}

afterEvaluate {
    publishing {
        repositories {
            maven {
                name = (project.findProperty("gpr.repo.name") ?: "GitHubPackages").toString()
                url = uri((project.findProperty("gpr.repo.url") ?: "https://maven.pkg.github.com/bitcoinppl.bbqr/bbqr-ffi").toString())
                credentials {
                    username = (project.findProperty("gpr.user") ?: System.getenv("GITHUB_ACTOR")).toString()
                    password = (project.findProperty("gpr.key") ?: System.getenv("GITHUB_TOKEN")).toString()
                }
            }
        }

        publications {
            create<MavenPublication>("maven") {
                groupId = "org.bitcoinppl"
                artifactId = "bbqr-android"
                version = libraryVersion

                from(components["release"])
                pom {
                    name.set("bbqr-android")
                    description.set("BBQr Kotlin language bindings.")
                    url.set("https://bitcoinppl.org")
                    licenses {
                        license {
                            name.set("APACHE 2.0")
                            url.set("https://github.com/bitcoinppl/bbqr/blob/master/LICENSE")
                        }
                    }
                    developers {
                        developer {
                            id.set("bitcoinppl")
                            name.set("Bitcoin PPL Developers")
                            email.set("me@praveenperera.com")
                        }
                    }
                    scm {
                        connection.set("scm:git:github.com/bitcoinppl/bbqr-ffi.git")
                        developerConnection.set("scm:git:ssh://github.com/bitcoinppl/bbqr-ffi.git")
                        url.set("https://github.com/bitcoinppl/bbqr-ffi/tree/master")
                    }
                }
            }


        }
    }
   
    // This is required because we must ensure the moveNativeAndroidLibs task is executed after
    // the mergeReleaseJniLibFolders (hard requirement introduced by our upgrade to Gradle 8.7)
    tasks.named("mergeReleaseJniLibFolders") {
        dependsOn(":lib:moveNativeAndroidLibs")
    }
    tasks.named("mergeDebugJniLibFolders") {
        dependsOn(":lib:moveNativeAndroidLibs")
    }
}

signing {
    if (project.hasProperty("localBuild")) {
        isRequired = false
    }
    
    val signingKeyId: String? = project.findProperty("signingKeyId") as String? ?: System.getenv("SIGNING_KEY_ID")
    var signingKey: String? = project.findProperty("signingKey") as String? ?: System.getenv("SIGNING_KEY")
    val signingPassword: String? = project.findProperty("signingPassword") as String? ?: System.getenv("SIGNING_PASSWORD")

    useInMemoryPgpKeys(signingKey, signingPassword)
    sign(publishing.publications)
}

// This task dependency ensures that we build the bindings binaries before running the tests
tasks.withType<KotlinCompile> {
    dependsOn("buildAndroidLib")
}
