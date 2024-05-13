# SCUBA Android Breadcrumbs

TODO when/how is `local.properties` generated? Contains $ANDROID_HOME value.
Currently checked into git but values are specific to my machine, so need 
instructions for generating it per machine. 




If python is not found (but python3 is installed), I got the simple app to build by simply:

```sh
sudo cp /usr/bin/python3 /usr/bin/python
```



For `olm-rs` crate need to set: 

```sh
export ANDROID_NDK="/home/np/Android/Sdk/ndk-bundle"
```



## Debugging `reqwest` crate issues

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



## Debugging `rkyv_codec` crate issues

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



## SCUBA libs

As of now, all dependencies compile. Next, let's test out SCUBA-specific code.
This will hopefully resolve two possible errors: (1) SCUBA code that may not compile
on different architectures, and (2) dependency version conflicts across all of core/tank's
dependencies. 

I create two softlinks, `core` (pointing to the scuba core library) and `tank`
(pointing to the tank library).

Let's first test out `core` (replace "simple" with "core" in `app/build.gradle` and
`/app/src/main/java/com/example/minimalrustimport/MainActivity.kt`).

TODO: remove all warnings from core/tank. 

Core works! Now let's try TANK. 

TANK works too! Woohooo!



## Building in Android Studio

The android studio build gets stuck at the ranlib errors above. I try editing
the project `gradle.properties` file with the environment variables that 
I set before (setting them in the Android Studio terminal tab did not work). 
Setting these values in the global `gradle.properties` does not work, nor does 
`gradle/wrapper/gradle-wrapper.properties`. 

From [this](https://stackoverflow.com/questions/75943717/error-building-rust-project-for-android-flutter-arm-linux-androideabi-ranlib)
link, setting the PATH vars in the AS terminal does not help. 

However, modifying the cargo config file does seem to do something. Namely, adding
at least environment variables $TOOLCHAIN and $RANLIB, as well as the linker
value for `aarch64-linux-android` works (see tank's top-level `/.cargo/config.toml`).

The app now *officially* builds _in_ Android Studio. 


## Debugging emulator errors

App compiles, but the emulator doesn't actually show the app. Launch apparently
succeeds but logcat shows the following error: 

`java.lang.UnsatisfiedLinkError: dlopen failed: library "libsimple.so" not found`.

Looking this up I find [this](https://stackoverflow.com/questions/52076641/java-lang-unsatisfiedlinkerror-dlopen-failed-library-not-found)
article. 

This is when I try to emulate the app remotely (X11 forwarding). To rule out
issues in that, I'll try to get things working locally on my Mac.


## Mac emulation

I install Android Studio Jellyfish (2023.3.1.18) while that on my desktop is Electric Eel (2022).

Configuring basic app to support minimum SDK API 24 ("Nougat"; Android 7.0). 

```sh
export ANDROID_HOME="~/Library/Android/sdk/"
```

(location found [here](https://stackoverflow.com/questions/19986214/setting-android-home-enviromental-variable-on-mac-os-x))

`./gradlew cargoBuild` apparently didn't like this, so I set the same value
in `local.properties` instead (key = sdk.dir).

Then I get an error saying that the NDK is not installed. The NDK (native 
development kit) apparently lets you use C/C++ with Android. I am not sure
why I need it though. Is this related to Rust? I install it via the Android
Studio SDK Manager. 

I also install Cmake and Android Studio command-line tools, since there seems
to be some dependency between NDK and Cmake, and Android Studio command-line tools
could be useful. 

Got the same error after trying to build again, but realized the `app/build.gradle` 
was still pointing to an old ndkVersion value, so I updated that and it worked. 

Don't forget to install all the necessary cross-comp toolchains: 

```sh
rustup target add armv7-linus-androideabi (Arm)
rustup target add aarch64-linux-android (Arm64)
rustup target add i686-linux-android (X86)
rustup target add x86_64-linux-android (X86_64)
```

Crates seem to be compiling now. Build fails with the following error: 

```sh
error: failed to run custom build command for `olm-sys v1.3.2`
...
please set the ANDROID_NDK environment variable to your Android NDK instalation
```

I'll try:

```sh
export ANDROID_NDK="~/Library/Android/sdk/ndk"
```

Then I get what is seemingly a Cmake configuration error. 

```sh
error: failed to run custom build command for `olm-sys v1.3.2`
...
-- Configuring incomplete, errors occurred!
...
CMake Error at...
  Could not find toolchain file:
  /Users/np/Library/Android/sdk/ndk/build/cmake/android.toolchain.cmake
Call Stack (most recent call first):
  CMakeLists.txt:3 (project)

CMake Error: CMake was unable to find a build program corresponding to "Unix Makefiles".  CMAKE_MAKE_PROGRAM is not set.  You probably need to select a different build tool.
CMake Error: CMAKE_CXX_COMPILER not set, after EnableLanguage
CMake Error: CMAKE_C_COMPILER not set, after EnableLanguage
...
/bin/sh: /home/np/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib: No such file or directory
...
Error installing OpenSSL:
```

This ranlib error looks familiar, which I fixed on Ubuntu using a `.cargo/config.toml` 
file. Ah, turns out that file is, of course, using paths specific to my desktop machine. 
So, I replace them with my Mac paths (to the SDK/NDK). TODO this file should
really be using $ANDROID_NDK but when I last tried it the variable wasn't 
properly resolved, which is why I'm using magic paths now. 

The above cmake error still occurs without the following ranlib ones. So at
least that was addressed. The cmake crate seems to be a dependency of olm-sys, which
is a dependency of olm-rs. 

From searching online, this error seems to mean that cmake is looking for make
and cannot find it. I have make installed and accessible on my path (`/usr/bin/make`)
so I'll try setting CMAKE_MAKE_PROGRAM to that path. 

That didn't help. 

I re-ran the build inside Android Studio (so far I've been doing both/just command-line)
and it succeeded in Android Studio. During that build it looked like a bunch of thing
were being installed as well. 

Ah, but the command-line version still does not work. Well, that's ok, we
were only using the command-line version to work without the Android Studio GUI,
but now that we have that and all its bells and whistles, we can just move forward there. 

Just kidding, Android Studio still fails to _run_ due to olm-sys:

```sh
please set the ANDROID_NDK environment variable to your Android NDK instalation
```

I try updating the `local.properties` file with the `ndk.dir` key. 
Build still fails, and I get a message saying that this method of setting NDK home 
is deprecated, so I set the ANDROID_NDK var in tank's `.cargo/config.toml`. 

cargoBuildArm64 and cargoBuildX86_64 fail running the openssl custom build script. 
Both outputs for these targets claim that openssl is successfully configured
(for cross-compilation I assume). But the both fail with:

```sh
make: *** read jobs pipe: Resource temporarily unavailable.  Stop.
```

(cargoBuildX86_64 error message also has a bunch of other lines printed out
unlike that for cargoBuildArm64). 

I'm not sure what exactly I changed, but running it again cargoBuildArm64
and cargoBuildX86_64 now both succeed. 

Now let's see how the emulator works. The app isn't really launching. My mac
only has 8GB RAM while 16 are recommended, which may explain this. I'll continue
testing this out once I get a physical android device from Leon on Monday. 




## Running Basic App (no Rust); Mac

An app that uses the BasicView Activity runs, but had a couple pop-ups that
claimed things were taking too long to load. Even after I clicked close app,
the app then worked? Unclear what was going on. 



## Running App with Simple (crate) Rust code

We'll comment out all of the simple lib dependencies we added for testing
SCUBA deps, so we can cut build time and increase iterations. 

Goal is to try to import Rust code that runs and produces output in the app UI. 
The default function implemented in a library created with `cargo new` is `add`
(adds two usizes together). So, in the app, we need to figure out how to (1) actually
import/call the add function (with arguments of course) _and_ (2) print out the
result. Let's focus on the first part. 

[Here](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
is an old post from Mozilla that I'm roughly following. It has some obsolete
things (i.e. `make_standalone_toolchain.py`, which explicity says "The compiler installed to
<NDK>/toolchains/llvm/prebuilt/<host>/bin can be used directly" which is what
we do in TANK's .cargo/config.toml file---this probably also needs to be done for
the `simple` library, but I'm not getting any build errors so maybe not). 

The above link has a function that prints "Hello <name>", so we can either
copy that or try to adapt it to `add`. Let's try to adapt it to `add` so 
we're not just blindly copying and can may get a better understanding of what's
going on. 

The following will be a bit of paraphrasing for the above link for my own understanding. 
Since our Rust will be called from non-Rust code, we'll need to use the foreign function
interface (FFI). I'm not sure _why_ but apparently we'll be calling the Rust code
specifically through a C bridge (I'm guessing the other option would be C++ or
some other language that can run natively?). Thus our function signature also needs
the `extern` keyword such that the function is compiled with respect to C
calling conventions. `#[no_mangle]` also does not mangle the function name so
we can easily call it from Android Studio code. 

Ah, it seems that a C ABI is commonly exposed by foreign code (i.e. various 
languages?), which is why we opt for a C bridge. Also, Rust apparently uses
the C ABI by default (i.e. if nothing is specified). 

Actually, let's write two functions. One with arguments, and one without. 

Note [this](https://doc.rust-lang.org/nomicon/ffi.html#asynchronous-callbacks) (async callbacks through FFI)
may be important in the future!

Now we have a `hello()` function that always returns "Hello from Rust!", and an
`add()` function that adds two integer arguments together and returns the result. 

```rust
use std::os::raw::{c_char};
use std::ffi::CString;

#[no_mangle]
pub extern fn hello() -> *mut c_char {
    CString::new("Hello from Rust!").unwrap().into_raw()
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
```



**Note:** Back on Linux.

The code compiles/launches, but run fails with `java.lang.UnsatisfiedLinkError: dlopen failed: library "libsimple.so" not found`. 
`libsimple.so` exists for each architecture I'm compiling for in the `app/build/rustJniLibs` directory. 
I tried to create `jniLibs` dir in `app/src/main` and put all the `.so`s in there,
but that didn't resolve the error. 

Hm, re-ran after deleting `jniLibs` and now it works.... Ok then!


Let us try to set the TextView that originally said "Hello World!" to the result
of our Rust `hello()` function. 

In a companion object (effectively a static class in Kotlin) in `MainActivity.kt`,
we have the following: 

```sh
init {
  System.loadLibrary("simple")
}
```

And our extern function declaration:

```sh
external fun hello(): String
```


**Note:** if get the following error, try to clean the app and re-build:

```sh
java.lang.UnsatisfiedLinkError: No implementation found for java.lang.String
```

We successfully run the app with the `hello()` function. Unfortunately, running
it with `add()` is still not working; most of the examples online only work with
strings while add works with integers, and it is not clear if this is the case
because working with objects is more difficult and there's a trick that I'm just
missing to be able to make things work with integers (and more than one parameter),
or if even integers must be treated as strings when interacting between languages. TBD. 




https://github.com/android/ndk-samples/blob/main/hello-jni/app/src/main/java/com/example/hellojni/HelloJni.kt

https://blog.svgames.pl/article/running-rust-on-android

https://github.com/suve/rust-on-android/blob/trunk/rust/src/android.rs

https://gendignoux.com/blog/2022/10/24/rust-library-android.html

https://users.rust-lang.org/t/using-jni-to-call-a-rust-function-that-passes-and-returns-a-struct/67488

