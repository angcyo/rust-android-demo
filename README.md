# 2025-02-05

`Android`使用`rust-android-gradle`进行`Rust`交叉编译.

https://github.com/mozilla/rust-android-gradle

另一个`demo`:

https://github.com/angcyo/AndroidRustDemo

## 1.在`root`的`build.gradle`文件中,添加插件

```diff
// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    alias(libs.plugins.android.application) apply false
    alias(libs.plugins.kotlin.android) apply false
+    id "org.mozilla.rust-android-gradle.rust-android" version "0.9.6"
}
```

低版本`AGP`使用:

```
dependencies {
    classpath 'org.mozilla.rust-android-gradle:plugin:0.9.6'
}
```

## 2.在需要项目中加入`cargo`配置

```diff
...
android { ... }

+ //--rust-android-gradle
+ // https://github.com/mozilla/rust-android-gradle

+ apply plugin: 'org.mozilla.rust-android-gradle.rust-android'

+ // https://github.com/mozilla/rust-android-gradle#configuration 配置项
+ cargo {
+     module  = "../rust"       // Or whatever directory contains your Cargo.toml
+     libname = "rust"          // Or whatever matches Cargo.toml's [package] name.
+     targets = ["arm", "arm64"]  // See bellow for a longer list of options
+     profile = "release"
+ }
```

安装对应的`Rust Toolchains`交叉编译工具链:

```
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
rustup target add x86_64-unknown-linux-gnu  # for linux-x86-64
rustup target add x86_64-apple-darwin       # for darwin x86_64 (if you have an Intel MacOS)
rustup target add aarch64-apple-darwin      # for darwin arm64 (if you have a M1 MacOS)
rustup target add x86_64-pc-windows-gnu     # for win32-x86-64-gnu
rustup target add x86_64-pc-windows-msvc    # for win32-x86-64-msvc
...
```

运行任务, 即可生成`so文件`:

```
./gradlew cargoBuild
```

自动构建需要如下配置依赖:

```
tasks.whenTaskAdded { task ->
    if ((task.name == 'javaPreCompileDebug' || task.name == 'javaPreCompileRelease')) {
        task.dependsOn 'cargoBuild'
    }
}
```

## 3.在项目中使用

```
static {
    System.loadLibrary("rust");
}
```

or

```
object Rust {
    init {
        System.loadLibrary("rust")
    }
}
```

## 配置输出目录

https://github.com/ncalexan/rust-android-gradle/blob/master/README.md#targetdirectory

默认输出在`${module}/target`文件夹中.

```diff
cargo {
    // Note: path is either absolute, or relative to the gradle project's `projectDir`.
+    targetDirectory = 'target'
}
```

# 常见问题

## 要将`java`类生成`jni`方法签名, 可以使用`javah`, jdk9之后使用`javac`.

https://docs.oracle.com/javase/9/tools/javah.htm

```
javac -d build -h build app/src/main/java/com/angcyo/rust_android_demo/Rust.java
```

建议创建一个`Native C++`工程, 然后`AS`就会自动提示创建`Create JNI function for xxx`的提示. 

## `kotlin`需要先安装命令行工具

https://kotlinlang.org/docs/command-line.html

> 没有成功, 建议使用java类创建jni签名.

## 如果`cargoBuild`出现以下错误:

```
Execution failed for task ':app:cargoBuildArm'.
> A problem occurred starting process 'command 'rustc''
> A problem occurred starting process 'command 'cargo''
```

则需要配置对应的可执行文件路径:

https://github.com/ncalexan/rust-android-gradle/blob/master/README.md#specifying-paths-to-sub-commands-python-cargo-and-rustc

```diff
cargo {
    ...
+    rustcCommand = "/Users/angcyo/.cargo/bin/rustc"
+    cargoCommand = "/Users/angcyo/.cargo/bin/cargo"
}
```

# 链接动态库`cdylib`时, 出现`python: command not found`错误:

```
error: linking with `/Users/angcyo/project/android/rust-android-demo/build/linker-wrapper/linker-wrapper.sh` failed: exit status: 127
  |
  = note: LC_ALL="C" PATH="/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin:/Users/angcyo/.pub-cache/bin:/opt/homebrew/opt/ruby/bin:/opt/homebrew/bin:/opt/homebrew/sbin:/usr/local/bin:/System/Cryptexes/App/usr/bin:/usr/bin:/bin:/usr/sbin:/sbin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/local/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/appleinternal/bin:/Library/Apple/usr/bin:/Users/angcyo/Library/Android/sdk/platform-tools:/Users/angcyo/Library/Android/flutter/bin:/Users/angcyo/.cargo/bin:/Users/angcyo/Library/Application Support/JetBrains/Toolbox/scripts:/Users/angcyo/Library/Android/flutter/bin" VSLANG="1033" "/Users/angcyo/project/android/rust-android-demo/build/linker-wrapper/linker-wrapper.sh" "-Wl,--version-script=/var/folders/vc/bjzygdd91899dm6bwfwnz2gh0000gn/T/rustcyed4OC/list" "-Wl,--no-undefined-version" "/var/folders/vc/bjzygdd91899dm6bwfwnz2gh0000gn/T/rustcyed4OC/symbols.o" "/Users/angcyo/project/android/rust-android-demo/rust/target/aarch64-linux-android/release/deps/rust.rust.f028cdb7c85fe392-cgu.0.rcgu.o" "/Users/angcyo/project/android/rust-android-demo/rust/target/aarch64-linux-android/release/deps/rust.492mv7sr0lj09drkxnejwq4y7.rcgu.o" "-Wl,--as-needed" "-Wl,-Bstatic" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libstd-47c013fe037d6a37.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libpanic_unwind-a7ad2f1aba2b106b.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libobject-d5d38bf2619e81ce.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libmemchr-5362d931fb1cc9e9.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libaddr2line-44ab3a3a8baebf90.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libgimli-f57d497095bfc250.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/librustc_demangle-1c52be4b4bb7e635.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libstd_detect-30dd52cd66d61366.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libhashbrown-4a7ba0d13f035ebb.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/librustc_std_workspace_alloc-c0aee6d3b1a1b56d.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libminiz_oxide-597ee827e0200569.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libadler-5816e85e91adec98.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libunwind-5e77e37da31b0e8b.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libcfg_if-500c35c6e9812e20.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/liblibc-73b6e01407d62091.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/liballoc-90d1c9c8a890764b.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/librustc_std_workspace_core-98e1c51044269be0.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libcore-7a41fc440ac35804.rlib" "/Users/angcyo/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-linux-android/lib/libcompiler_builtins-9f190a0c4f1f4fd3.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lunwind" "-ldl" "-lm" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-o" "/Users/angcyo/project/android/rust-android-demo/rust/target/aarch64-linux-android/release/deps/librust.so" "-Wl,--gc-sections" "-shared" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-Wl,--strip-debug" "-nodefaultlibs"
  = note: /Users/angcyo/project/android/rust-android-demo/build/linker-wrapper/linker-wrapper.sh: line 4: python: command not found
```

```diff
cargo {
    ...
+    pythonCommand = "/usr/bin/python3"    
}
```

## Generated apk does not contain jniLibs

https://github.com/mozilla/rust-android-gradle/issues/147

## Automatically bundle libc++_shared in apk?

https://github.com/mozilla/rust-android-gradle/issues/106