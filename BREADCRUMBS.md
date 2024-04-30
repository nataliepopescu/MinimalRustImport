If python is not found (but python3 is installed), I got the simple app to build by simply:

```sh
sudo cp /usr/bin/python3 /usr/bin/python
```



App compiles, but the emulator doesn't actually show the app. Launch apparently
succeeds but logcat shows the following error: 

`java.lang.UnsatisfiedLinkError: dlopen failed: library "libsimple.so" not found`.

Looking this up I find [this](https://stackoverflow.com/questions/52076641/java-lang-unsatisfiedlinkerror-dlopen-failed-library-not-found)
article. 

TODO still need to fix this. 



For `olm-rs` crate need to set: 

```sh
export ANDROID_NDK="/home/np/Android/Sdk/ndk-bundle"
```



# Debugging `reqwest` crate issues

```sh
error: failed to run custom build command for `openssl-sys v0.9.102`
...
run pkg_config fail: pkg-config has not been configured to support cross-compilation
```

Added `openssl = { version = "0.10.64", features = ["vendored"] }` to simple's Cargo.toml. 

Then:

```sh
error: failed to run custom build command for `openssl-sys v0.9.102`
...
/bin/sh: 1: arm-linux-androideabi-ranlib: not found
...
> Task :app:cargoBuildArm FAILED
```

Using [this](https://stackoverflow.com/questions/75943717/error-building-rust-project-for-android-flutter-arm-linux-androideabi-ranlib)
link, I'll try setting the following (checking `./gradlew cargoBuild` after each one):

```sh
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653/

export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64
export TARGET=aarch64-linux-android
export API=33

export AR=$TOOLCHAIN/bin/llvm-ar
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export AS=$CC
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/ld
export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
export STRIP=$TOOLCHAIN/bin/llvm-strip
```

However, I think this will probably only work for one toolchain target (aarch64-linux-android)

Update: cargoBuildArm, cargoBuildArm64, cargoBuild86, and cargoBuildX86_64 all work! 
(Note, I didn't have to make the PATH or `$HOME/.cargo/config` changes listed in the link).

I double checked cargoBuildX86 without these envvars set, which has a similar error:

```sh
/bin/sh: 1: i686-linux-android-ranlib: not found
```

So something about the RANLIB/TOOLCHAIN vars must have helped. (TARGET is hopefully only set per target)



# Debugging `rkyv_codec` crate issues

```sh
error[E0308]: mismatched types
  --> /home/np/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rkyv_codec-0.3.1/src/length_codec.rs:26:42
   |
26 |         unsigned_varint::encode::usize(length, buf)
   |         ------------------------------         ^^^ expected an array with a fixed size of 5 elements, found one with 10 elements
   |         |
   |         arguments to this function are incorrect
   |
note: function defined here
  --> /home/np/.cargo/registry/src/index.crates.io-6f17d22bba15001f/unsigned-varint-0.7.2/src/encode.rs:95:8
   |
95 | pub fn usize(number: usize, buf: &mut [u8; USIZE_LEN]) -> &[u8] {
   |        ^^^^^

For more information about this error, try `rustc --explain E0308`.
```

In looking at what `rkyv_codec` is actually used for in the server, turns
out all related code is commented out. So, no need to compile this crate anyway!



As of now, all dependencies compile. Next, let's test out SCUBA-specific code.













