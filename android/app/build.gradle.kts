plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "dev.luca.phoneconnect"
    compileSdk = 36

    defaultConfig {
        applicationId = "dev.luca.phoneconnect"
        // Android 10: da qui in poi valgono le restrizioni sugli appunti che gestiamo.
        minSdk = 29
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"))
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
