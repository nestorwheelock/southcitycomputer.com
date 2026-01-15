# South City Computer Android App

A simple WebView-based Android app that opens southcitycomputer.com.

## Features

- Splash screen with gradient branding
- Full WebView with JavaScript support
- Progress bar during page loads
- Back button navigation within the app
- Handles orientation changes

## Building

### Option 1: Android Studio (Recommended)
1. Open Android Studio
2. File → Open → Select this `android-app` folder
3. Wait for Gradle sync
4. Build → Build Bundle(s) / APK(s) → Build APK(s)
5. APK will be in `app/build/outputs/apk/release/`

### Option 2: Command Line
```bash
# Make sure ANDROID_HOME is set
export ANDROID_HOME=/path/to/android/sdk

# Build debug APK
./gradlew assembleDebug

# Build release APK (requires signing)
./gradlew assembleRelease
```

## Signing for Release

Create a keystore:
```bash
keytool -genkey -v -keystore southcitycomputer.keystore -alias scc -keyalg RSA -keysize 2048 -validity 10000
```

Add to `app/build.gradle`:
```gradle
android {
    signingConfigs {
        release {
            storeFile file('southcitycomputer.keystore')
            storePassword 'your-password'
            keyAlias 'scc'
            keyPassword 'your-password'
        }
    }
    buildTypes {
        release {
            signingConfig signingConfigs.release
        }
    }
}
```

## Deployment

After building, copy the APK to:
```
../app/SouthCityComputer.apk
```

The download page is at: https://southcitycomputer.com/app/
