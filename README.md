# Prismer

Rust bindings to [prism](https://github.com/ethindp/prism), the platform-agnostic reader interface for speech and messages. Prism unifies screen readers and TTS engines (SAPI, NVDA, JAWS, VoiceOver, AVSpeech, speech-dispatcher, Orca, Android TTS, WebSpeech, and more) behind one API, so this crate gives your Rust application speech, braille, and screen reader output on every major platform.

## Crates

- `prism-sys`: raw FFI bindings to the prism C API, `no_std`.
- `prismer`: safe idiomatic wrapper.

## Quick start

```rust
let prism = prismer::Prism::new()?;
let backend = prism.create_best()?;
backend.speak("Hello from Rust!", false)?;
```

See `crates/prismer/examples/speak.rs` for a fuller example, including feature detection and waiting for speech to finish.

## Picking a backend

There are four ways to get a backend:

| Method | Backend state | Initialized on return |
| --- | --- | --- |
| `create_best()` | private to you | yes |
| `create(id)` | private to you | no, call `initialize()` |
| `acquire_best()` | shared with other callers | yes |
| `acquire(id)` | shared with other callers | maybe, call `initialize()` and treat `AlreadyInitialized` as success |

Use `create_best()` unless you have a specific reason not to. The `acquire` family returns a cached instance, so a voice, rate, or pitch set through one handle is visible through every other handle to that backend. Reach for it only when sharing that state is what you want.

## Building

Prism is vendored as a git submodule and built automatically by `prism-sys`'s build script, so all you need is a C++23 toolchain and CMake 3.24+:

```sh
git clone --recurse-submodules https://github.com/accessware/prismer
cd prismer
cargo build
```

If you cloned without `--recurse-submodules`, run `git submodule update --init` first.

To link against a prebuilt prism instead (skipping the CMake build), set `PRISM_LIB_DIR` to the directory containing the library:

```sh
set PRISM_LIB_DIR=path\to\prism\build
cargo build
```

By default prism is built and linked as a shared library; note that your application must be able to find it (next to the executable on Windows, or on the loader path elsewhere) at runtime. Enable the `static` feature to build and link it statically instead. iOS, tvOS, watchOS and visionOS always link it statically, and Android always links it shared, whatever the feature says. The build script links the C++ runtime and the system libraries prism needs for you.

On Apple platforms that only holds when cargo does the final link. If you build a Rust `staticlib` and link it from Xcode, add the system frameworks to your Xcode target yourself: Foundation and AVFoundation everywhere, plus AppKit, CoreServices, Security, IOKit and CoreFoundation on macOS, UIKit on iOS, tvOS and visionOS, or WatchKit on watchOS. prism's Objective-C++ also needs `libobjc`, which Xcode links for any target with Objective-C or Swift code of its own.

## Platforms

CI builds and tests every target below, and runs the iOS and Android examples in a simulator and an emulator.

| Platform | Targets | Backends | Oldest version |
| --- | --- | --- | --- |
| Windows | `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`, `i686-pc-windows-msvc` | NVDA, JAWS, ZoomText, SAPI, OneCore, UI Automation and more | Windows 10 |
| macOS | `aarch64-apple-darwin`, `x86_64-apple-darwin` | VoiceOver, AVSpeech | macOS 11 |
| iOS | `aarch64-apple-ios`, `aarch64-apple-ios-sim` | VoiceOver, AVSpeech | iOS 14 |
| Linux | `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` | Speech Dispatcher, Orca | The build machine's glibc |
| Android | `aarch64-linux-android`, `x86_64-linux-android` | The running screen reader, such as TalkBack, and Android text to speech | Android 8 (API 26) |

### Apple

prism needs `std::jthread`, which Apple's C++ library only has from macOS 11 and iOS 14 on, so prism is built for those or for whatever `MACOSX_DEPLOYMENT_TARGET` or `IPHONEOS_DEPLOYMENT_TARGET` asks for. rustc links iOS devices for iOS 10 and Intel Macs for macOS 10.12 unless told otherwise, so set the variable for those, for example `IPHONEOS_DEPLOYMENT_TARGET=14.0`. Without it an iOS device build fails to link, and the build script warns you. Xcode sets it for you when it runs cargo from a build phase.

### Linux

prism uses `libdbus-1`, and talks to Speech Dispatcher, which has to be installed with a voice, as it is on most desktops. Building needs the development files for D-Bus and Speech Dispatcher, `libdbus-1-dev` and `libspeechd-dev` on Ubuntu, and prism's Clipboard backend also wants `libxcb1-dev` and `libwayland-dev`.

### Android

prism's Android backends are written in Java as much as C++, so an Android app needs prism's library archive as well as your Rust library:

1. Build your Rust library with [cargo-ndk](https://github.com/bbqsrc/cargo-ndk), which hands prism-sys the NDK. On Windows, `ninja` has to be on `PATH`.
2. Build prism's archive from the same prism prism-sys was built from, with `gradlew assembleRelease` in the `prism` submodule, and add it to your app with core library desugaring enabled. Besides the Java backends, it carries `libprism.so` for each ABI, and a manifest entry that hands prism the application context when the app starts.
3. Load prism from a Java or Kotlin class of your app before your Rust library, with `System.loadLibrary("prism")`. prism finds its Java classes through the class loader of whatever loads it.
4. Call `create_best`, `acquire_best` and `initialize` from a thread other than the main one. Starting Android's speech engine waits on a callback that arrives on the main thread, so on the main thread it times out and the backend fails to start.

`examples/android` is a complete app doing all of this. With prism's archive built, `gradlew assembleDebug` there builds the Rust library and the app together.

## Custom backends

You can write a backend in Rust and register it alongside prism's own. Implement `CustomBackend`, declare the operations you implemented, and freeze a registry:

```rust
use prismer::{CustomBackend, Features, Prism, RegistryBuilder};

struct Printer;

impl CustomBackend for Printer {
	fn speak(&mut self, text: &str, interrupt: bool) -> prismer::Result<()> {
		println!("{text}");
		Ok(())
	}
}

let mut builder = RegistryBuilder::new()?;
let id = builder.add_backend("Example Printer", 10, Features::SPEAK, || Some(Printer))?;
let registry = builder.freeze()?;
let prism = Prism::builder().registry(&registry).build()?;
let backend = prism.create(id)?;
```

Every trait method defaults to `NotImplemented`, and the vtable handed to prism gets a pointer only for the features you declare, so the two can never disagree. The factory closure runs once per instance, so instances never share state. See `crates/prismer/examples/custom_backend.rs` for a full program.

`RegistryBuilder::add_library` loads a prism plugin shared library and registers the backends it supplies.

## Logging

The `log` module wraps prism's process-wide logger:

```rust
prismer::log::set_level(prismer::log::Level::Warn);
prismer::log::set_handler(|level, source, message| {
	eprintln!("[{level:?}] {source}: {message}");
});
```

The handler runs on prism's logging thread, so keep it quick. It is leaked on purpose: prism may still deliver to a replaced handler, so there is no safe moment to free one.

## Status

Early draft, but the C API is fully covered: context init and configuration (including availability callbacks), registry queries, backend creation and acquisition, speech, braille, output, playback control, volume, rate, pitch, voice enumeration and selection, audio format queries, speak-to-memory with a closure callback, custom backends and plugin libraries through the registry builder, and the logging API.

## License

MIT. Prism itself is MPL-2.0.
