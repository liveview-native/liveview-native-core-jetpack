plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("maven-publish")
    id("org.mozilla.rust-android-gradle.rust-android")
}

android {
    namespace = "org.phoenixframework.liveview_native_core_jetpack"
    compileSdk = 33

    defaultConfig {
        minSdk = 21
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    publishing {
        singleVariant("release") {
            withSourcesJar()
            withJavadocJar()
        }
    }
    ndkVersion = "25.1.8937393"
}

// Configuring Rust Cargo build
// https://github.com/mozilla/rust-android-gradle
cargo {
    module = "../jni_bindings"
    libname = "liveview_native_core"
    targets = listOf("arm", "arm64", "x86", "x86_64", "darwin-x86-64", "darwin-aarch64")
}

dependencies {
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
}

// Running cargo command before build
tasks.configureEach {
    if ((name == "javaPreCompileDebug" || name == "javaPreCompileRelease")) {
        dependsOn("cargoBuild")
    }
}

// Configuring Java Lib Path in order to find the native library before running the Unit Tests
tasks.withType<Test>().configureEach {
    doFirst {
        val rustJniLibsForDesktopDir = File("${projectDir}/build/rustJniLibs/desktop")
        val archTypesSubdirs = rustJniLibsForDesktopDir.listFiles()
        for (dir in archTypesSubdirs) {
            // Selecting the proper JNI lib file in according to the architecture
            // e.g.: darwin-aarch64, darwin-x86-64
            val arch = System.getProperty("os.arch").replace("_", "-")
            if (dir.isDirectory && dir.name.contains(arch)) {
                systemProperty("java.library.path", dir.absolutePath)
                break
            }
        }
    }
}

publishing {
    publications {
        register<MavenPublication>("release")  {
            groupId = "org.phoenixframework"
            artifactId = "liveview-native-core-jetpack"
            version = "0.1.0-pre-alpha-08"

            afterEvaluate {
                from(components["release"])
            }
        }
    }
}