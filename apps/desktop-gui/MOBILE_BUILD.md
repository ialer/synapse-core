# SynapseCore - Mobile Build Configuration

## Prerequisites

### Android

1. **Android Studio** (latest stable)
   - Download: https://developer.android.com/studio
   - Install with default settings

2. **Android SDK** (minimum API 24, target API 34)
   - Install via Android Studio SDK Manager
   - Required SDK components:
     - Android SDK Platform 34
     - Android SDK Build-Tools 34.0.0
     - Android SDK Command-line Tools
     - Android Emulator (for testing)
     - Android SDK Platform-Tools

3. **Environment Variables**
   ```bash
   export ANDROID_HOME="$HOME/Android/Sdk"
   export PATH="$PATH:$ANDROID_HOME/emulator"
   export PATH="$PATH:$ANDROID_HOME/platform-tools"
   export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
   ```

4. **Java Development Kit (JDK) 17**
   ```bash
   # Ubuntu/Debian
   sudo apt install openjdk-17-jdk

   # macOS
   brew install openjdk@17
   ```

### iOS (macOS only)

1. **Xcode** (14.0 or later)
   - Download from Mac App Store
   - Install Command Line Tools: `xcode-select --install`

2. **CocoaPods**
   ```bash
   sudo gem install cocoapods
   ```

3. **Rust targets**
   ```bash
   rustup target add aarch64-apple-ios
   rustup target add x86_64-apple-ios
   ```

---

## Initialization

### Android

```bash
cd apps/desktop-gui

# Initialize Android project
npm run android:init

# This generates:
# - src-tauri/gen/android/ (Android project structure)
# - Integrates with your existing tauri.conf.json
```

### iOS

```bash
cd apps/desktop-gui

# Initialize iOS project
npm run ios:init

# This generates:
# - src-tauri/gen/apple/ (Xcode project)
# - Integrates with your existing tauri.conf.json
```

---

## Development

### Android

```bash
# Run on connected device or emulator
npm run android:dev

# Or use tauri CLI directly
tauri android dev
```

**Steps:**
1. Start Android Emulator or connect a device via USB
2. Run `npm run android:dev`
3. App will hot-reload on changes

### iOS

```bash
# Run on simulator or connected device
npm run ios:dev

# Or use tauri CLI directly
tauri ios dev
```

**Steps:**
1. Open Xcode, select target simulator/device
2. Run `npm run ios:dev`
3. App will hot-reload on changes

---

## Building

### Android APK

```bash
# Debug build (for testing)
npm run android:build:debug

# Release build (for distribution)
npm run android:build:release
```

**Output locations:**
- Debug: `src-tauri/gen/android/app/build/outputs/apk/debug/`
- Release: `src-tauri/gen/android/app/build/outputs/apk/release/`

**For Google Play Store:**
- Use release build
- Sign with your keystore
- Follow Play Store guidelines for AAB format

### iOS App

```bash
# Debug build
npm run ios:build:debug

# Release build
npm run ios:build:release
```

**For App Store:**
1. Open `src-tauri/gen/apple/SynapseCore.xcworkspace`
2. Configure signing in Xcode
3. Archive and upload via Xcode Organizer

---

## Configuration Files

### tauri.conf.json (Mobile Settings)

The `tauri.conf.json` has been updated with mobile-specific configuration:

```json
{
  "android": {
    "minSdkVersion": 24,
    "targetSdkVersion": 34
  },
  "iOS": {
    "minimumDeploymentTarget": "14.0"
  }
}
```

### Android Manifest

Located at: `src-tauri/gen/android/app/src/main/AndroidManifest.xml`

Includes:
- Internet permissions
- Network state access
- File storage permissions (API < 29)
- Fullscreen activity configuration

### iOS Info.plist

Located at: `src-tauri/gen/apple/Info.plist`

Includes:
- Camera access (QR scanning)
- Photo library access
- Face ID usage
- Local network access (Bonjour)
- Landscape + Portrait orientations

---

## Troubleshooting

### Common Issues

1. **"ANDROID_HOME not set"**
   - Set environment variables as shown in Prerequisites

2. **"No emulator found"**
   - Start Android Studio → AVD Manager → Create/Start emulator

3. **"CocoaPods not found"**
   - Run `sudo gem install cocoapods`
   - Then `cd src-tauri/gen/apple && pod install`

4. **"Rust target not installed"**
   ```bash
   rustup target add aarch64-apple-ios
   rustup target add x86_64-apple-ios
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   ```

5. **"Build failed - SDK version mismatch"**
   - Ensure Android SDK 34 is installed
   - Check `local.properties` in Android project

### Debug Logs

**Android:**
```bash
adb logcat | grep -i synapse
```

**iOS:**
- Check Xcode Console for logs
- Or use: `log stream --predicate 'process == "SynapseCore"'`

---

## Project Structure

```
apps/desktop-gui/
├── src-tauri/
│   ├── tauri.conf.json          # Main Tauri config (with mobile settings)
│   ├── Cargo.toml               # Rust dependencies
│   ├── src/
│   │   └── main.rs              # Tauri app entry point
│   └── gen/
│       ├── android/             # Android project (generated)
│       │   ├── app/
│       │   │   ├── build.gradle.kts
│       │   │   └── src/main/
│       │   │       ├── AndroidManifest.xml
│       │   │       ├── kotlin/.../MainActivity.kt
│       │   │       └── res/values/styles.xml
│       │   ├── build.gradle.kts
│       │   ├── settings.gradle.kts
│       │   └── gradle.properties
│       └── apple/               # iOS project (generated)
│           └── Info.plist
├── package.json                 # NPM scripts (with mobile commands)
└── MOBILE_BUILD.md              # This file
```

---

## Next Steps

1. Run `npm run android:init` to generate Android project files
2. Run `npm run ios:init` to generate iOS project files (macOS only)
3. Test on emulators/simulators
4. Configure app signing for production builds
5. Set up CI/CD for automated mobile builds

---

## Resources

- Tauri 2.0 Mobile Guide: https://v2.tauri.app/mobile/
- Android Developer: https://developer.android.com
- Apple Developer: https://developer.apple.com
- Tauri GitHub: https://github.com/tauri-apps/tauri
