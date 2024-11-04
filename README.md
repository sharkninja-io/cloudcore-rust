# CloudCore Rust Library

The CloudCore Rust library consist of 4 packages (In order of outer visibility):
- **cloudcore**: Consists of the actual calls to Ayla and any other additional business logic.
- **ffi**: Wraps the ***cloudcore*** package in a C friendly API. This package produces either static (iOS) or shared (Android) library.
- **android**: Calls the functions exposed by the ***ffi*** package. Also provides an interface for Kotlin/Java to call the exposed ***ffi*** functions. This package produces a shared library.
- **ios**: Calls the functions exposed by the ***ffi*** package. Also provides an interface for Swift (via C) to call the exposed ***ffi*** functions. This package produces a static library.

### Install Rust:
```curl https://sh.rustup.rs -sSf | sh```

### Setup for iOS:
1. Install Xcode from the Mac App Store
2. Run the following in terminal:
   1. ```xcode-select --install```
   2. ```rustup target add aarch64-apple-ios x86_64-apple-ios```
   3. ```cargo install cargo-lipo```

### Build for iOS:
1. In terminal ```cd ios```
2. To make the universal release static lib: ```cargo lipo --release```. To make a debug version just leave off the ```release``` flag.
3. To make static libs for a specific target (real device): ```cargo build --target aarch64-apple-ios```, (simulator): ```cargo build --target x86_64-apple-ios```
4. In terminal ```cd ../ffi```
5. Repeat step **3** or **4** depending on if a specific target or fat binary is desired.

### Setup for Android:
1. Go to Android Studio > Preferences > Appearance & Behaviour > System Settings > Android SDK > SDK Tools.
2. Click Show Package Details > NDK.
3. Install the latest r22 version of the NDK (22.1.7171670). This seems to be the last version that enables the standalone toolchain *with gcc*.
4. Install Android SDK API level 31
5. Install the latest CMake
6. Choose a folder where you want to install the NDK. Open the folder in a terminal and type:
   1. ```export ANDROID_HOME=/Users/$USER/Library/Android/sdk``` 
   2. ```export NDK_HOME=${ANDROID_HOME}/ndk/22.1.7171670``` 
   3. ```mkdir NDK```
   - *The following commands may take a little time to complete
   4. ```${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 31 --arch arm64 --install-dir NDK/arm64```
   5. ```${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 31 --arch arm --install-dir NDK/arm```
   6. ```${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 31 --arch x86 --install-dir NDK/x86```
   7. ```${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 31 --arch x86_64 --install-dir NDK/x86_64```
   8. ```rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android```
7. Create a symbolic link to the NDK in the project root directory named NDK. For example if the NDK folder was created at ***/Users/$USER/NDK***:
   - ```ln -s /Users/$USER/NDK ./NDK```
8. Add the NDK bin folder to the PATH environment variable. For example if you installed the NDK at ***/Users/$USER/NDK***:
   ```Sh
   export PATH=$PATH:/Users/$USER/NDK/x86/bin/:/Users/$USER/NDK/arm/bin/:/Users/$USER/NDK/arm64/bin/:/Users/$USER/NDK/x86_64/bin/
   ```
9. Make sure the system bin folders are in there as well (at least the ones that exist 🙂 )
   ```Sh
   export PATH=$PATH:/usr/bin:/bin:/usr/sbin:/sbin
   ```
   - You can also use other ways to make sure those paths are in the PATH environment variable as well.
   - ***Note**: If you only set the PATH variable for the session you will need to repeat steps **8** & **9** each time. It is best to set the PATH variable in a persistent way. If you do permanently add them to the PATH environment variable make sure to restart your terminal session. 
10. Install Xcode from the Mac App Store
11. Run the following in terminal:
    1. ```xcode-select --install```

### Build for Android:
***Note**: Leave off ```release``` flag for debug versions of shared lib.
1. In terminal ```cd android```
2. For x86: ```cargo build --target i686-linux-android --release```
3. For armeabi-v7a: ```cargo build --target armv7-linux-androideabi --release```
4. For arm64-v8a: ```cargo build --target aarch64-linux-android --release```
5. In terminal ```cd ../ffi```
6. Repeat steps **2** - **4**
