//! Builds and links the vendored prism library.

use std::{
	env, fs,
	path::{Path, PathBuf},
};

use cmake::Config;

/// The Windows system libraries prism links itself, in `PrismPlatformWindows.cmake`.
///
/// A static prism leaves these to the final link too, and nothing else names them there. An
/// application only linked them by luck, when another dependency happened to pull them in;
/// otherwise it failed with unresolved UI Automation, RPC and Windows Runtime symbols.
const WINDOWS_SYSTEM_LIBS: &[&str] = &["ole32", "onecore", "runtimeobject", "uiautomationcore", "rpcrt4", "powrprof"];

/// The oldest release of each Apple OS prism builds for, with the variable that sets it.
///
/// prism needs `std::jthread`, which Apple's libc++ only has from these releases on.
const APPLE_DEPLOYMENT_TARGETS: &[(&str, &str, (u32, u32))] = &[
	("macos", "MACOSX_DEPLOYMENT_TARGET", (11, 0)),
	("ios", "IPHONEOS_DEPLOYMENT_TARGET", (14, 0)),
	("tvos", "TVOS_DEPLOYMENT_TARGET", (14, 0)),
	("watchos", "WATCHOS_DEPLOYMENT_TARGET", (7, 0)),
	("visionos", "XROS_DEPLOYMENT_TARGET", (1, 0)),
];

/// The oldest Android API level prism supports, from its Gradle build's `minSdk`.
const ANDROID_PLATFORM: &str = "android-26";

fn main() {
	println!("cargo:rerun-if-env-changed=PRISM_LIB_DIR");
	let static_link = match env::var("CARGO_CFG_TARGET_OS").as_deref() {
		// An app on these can't load a library cargo built beside it, so prism goes into the app.
		Ok("ios" | "tvos" | "watchos" | "visionos") => true,
		// prism's Android backends are Java as much as C++, so Java has to load prism itself, to run its JNI_OnLoad.
		Ok("android") => false,
		_ => env::var("CARGO_FEATURE_STATIC").is_ok(),
	};
	let vendored = env::var("PRISM_LIB_DIR").map_or_else(
		|_| Some(build_vendored(static_link)),
		|dir| {
			println!("cargo:rustc-link-search=native={dir}");
			None
		},
	);
	let kind = if static_link { "static" } else { "dylib" };
	println!("cargo:rustc-link-lib={kind}=prism");
	if static_link {
		if let Some(dst) = vendored {
			link_pkg_config_deps(&dst);
		}
		link_cpp_runtime();
		link_windows_system_libs();
		link_apple_frameworks();
	}
}

/// Links the Windows system libraries prism imports, which a shared prism
/// links itself and a static one leaves to whoever is doing the final link.
fn link_windows_system_libs() {
	if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
		return;
	}
	for lib in WINDOWS_SYSTEM_LIBS {
		println!("cargo:rustc-link-lib=dylib={lib}");
	}
}

/// Links the pkg-config modules prism builds against on Linux, such as giomm, which a static prism also leaves to the final link.
///
/// They're read from the `CMake` package prism installs, since which ones it uses depends on what the build machine has.
fn link_pkg_config_deps(dst: &Path) {
	let Ok(config) = fs::read_to_string(dst.join("share/prism/prism-config.cmake")) else {
		return;
	};
	for line in config.lines().filter(|line| line.trim_start().starts_with("pkg_check_modules(")) {
		let Some(spec) = line.split('"').nth(1) else {
			continue;
		};
		let name = spec.split(['>', '<', '=']).next().unwrap_or(spec);
		let mut probe = pkg_config::Config::new();
		if let Some(version) = spec[name.len()..].strip_prefix(">=") {
			let _ = probe.atleast_version(version);
		}
		if let Err(e) = probe.probe(name) {
			panic!("a static prism needs {spec} from pkg-config: {e}");
		}
	}
	if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
		println!("cargo:rustc-link-lib=dylib=dl");
	}
}

/// Links the frameworks prism links on Apple platforms, in `PrismPlatformApple.cmake`, which a static prism leaves to the final link too.
fn link_apple_frameworks() {
	let platform: &[&str] = match env::var("CARGO_CFG_TARGET_OS").as_deref() {
		Ok("macos") => &["AppKit", "CoreServices", "Security", "IOKit", "CoreFoundation"],
		Ok("ios" | "tvos" | "visionos") => &["UIKit"],
		Ok("watchos") => &["WatchKit"],
		_ => return,
	};
	for framework in ["Foundation", "AVFoundation"].iter().chain(platform) {
		println!("cargo:rustc-link-lib=framework={framework}");
	}
	// prism's Objective-C++ uses ARC, whose runtime functions clang adds only when it does the final link itself.
	println!("cargo:rustc-link-lib=dylib=objc");
}

fn build_vendored(static_link: bool) -> PathBuf {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
	let source = manifest_dir.join("prism");
	assert!(
		source.join("CMakeLists.txt").exists(),
		"vendored prism source not found at {}; run `git submodule update --init` \
		 or set PRISM_LIB_DIR to a directory containing a prebuilt prism library",
		source.display()
	);
	println!("cargo:rerun-if-changed=prism/CMakeLists.txt");
	println!("cargo:rerun-if-changed=prism/cmake");
	println!("cargo:rerun-if-changed=prism/source");
	println!("cargo:rerun-if-changed=prism/include");
	let mut cfg = Config::new(&source);
	let _ = cfg.define("BUILD_SHARED_LIBS", if static_link { "OFF" } else { "ON" });
	if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
		// prism defaults to the static CRT, but the objects must use the same
		// CRT rustc links: the non-debug one, static or dynamic per the
		// crt-static target feature.
		let crt_static =
			env::var("CARGO_CFG_TARGET_FEATURE").is_ok_and(|features| features.split(',').any(|f| f == "crt-static"));
		let _ = cfg.define("CMAKE_MSVC_RUNTIME_LIBRARY", if crt_static { "MultiThreaded" } else { "MultiThreadedDLL" });
	}
	set_apple_deployment_target(&mut cfg);
	use_android_ndk(&mut cfg);
	let dst = cfg.build();
	println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
	println!("cargo:rustc-link-search=native={}", dst.join("bin").display());
	dst
}

fn link_cpp_runtime() {
	let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
	let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
	match (os.as_str(), target_env.as_str()) {
		// MSVC objects carry /DEFAULTLIB directives for their C++ runtime.
		(_, "msvc") => {}
		("macos" | "ios" | "tvos" | "watchos" | "visionos", _) => println!("cargo:rustc-link-lib=c++"),
		_ => println!("cargo:rustc-link-lib=stdc++"),
	}
}

/// Builds prism for the Apple OS release rustc links for, or prism's oldest when that's older.
///
/// Without this, cc, which cmake-rs takes its compiler flags from, builds for the SDK's own
/// release, so the result only ran on the OS it was built with, and iOS devices failed to link
/// with `___chkstk_darwin` missing.
fn set_apple_deployment_target(cfg: &mut Config) {
	let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
	let Some(&(_, var, oldest)) = APPLE_DEPLOYMENT_TARGETS.iter().find(|(name, ..)| *name == os) else {
		return;
	};
	println!("cargo:rerun-if-env-changed={var}");
	let oldest_text = format!("{}.{}", oldest.0, oldest.1);
	let target = if let Ok(asked) = env::var(var) {
		assert!(
			parse_version(&asked) >= oldest,
			"{var} is {asked}, but prism needs at least {oldest_text}, the first release whose libc++ has std::jthread"
		);
		asked
	} else {
		if let Some(linked) = rustc_deployment_target().filter(|linked| parse_version(linked) < oldest) {
			println!(
				"cargo:warning=rustc links for {os} {linked}, but prism needs {oldest_text}. Set {var}={oldest_text} or \
				 later, or the link fails or the program fails to start on releases before {oldest_text}."
			);
		}
		oldest_text
	};
	// SAFETY: build scripts are single threaded, and nothing else reads the environment while this runs.
	unsafe { env::set_var(var, &target) };
	let _ = cfg.define("CMAKE_OSX_DEPLOYMENT_TARGET", &target);
}

/// Asks rustc which OS release it links the target for, which it prints as `VAR=version`.
fn rustc_deployment_target() -> Option<String> {
	let rustc = env::var_os("RUSTC")?;
	let target = env::var("TARGET").ok()?;
	let output =
		std::process::Command::new(rustc).args(["--print", "deployment-target", "--target", &target]).output().ok()?;
	let text = String::from_utf8(output.stdout).ok()?;
	Some(text.trim().split_once('=')?.1.to_owned())
}

fn parse_version(version: &str) -> (u32, u32) {
	let mut parts = version.split('.').map(|part| part.parse().unwrap_or(0));
	(parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

/// Builds prism with the NDK's own `CMake` toolchain, which cmake-rs then leaves the compiler to.
fn use_android_ndk(cfg: &mut Config) {
	if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
		return;
	}
	// cargo-ndk names the toolchain file itself. Otherwise it's found in the NDK.
	let vars = ["CARGO_NDK_CMAKE_TOOLCHAIN_PATH", "ANDROID_NDK_HOME", "ANDROID_NDK_ROOT", "ANDROID_NDK"];
	for var in vars {
		println!("cargo:rerun-if-env-changed={var}");
	}
	let toolchain = env::var_os(vars[0]).map(PathBuf::from).or_else(|| {
		vars[1..].iter().find_map(env::var_os).map(|ndk| PathBuf::from(ndk).join("build/cmake/android.toolchain.cmake"))
	});
	let toolchain = toolchain.expect(
		"building prism for Android needs the NDK; build with cargo-ndk, or set ANDROID_NDK_HOME to the NDK's directory",
	);
	let abi = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
		Ok("aarch64") => "arm64-v8a",
		Ok("arm") => "armeabi-v7a",
		Ok("x86_64") => "x86_64",
		Ok("x86") => "x86",
		other => panic!("prism has no Android ABI for {other:?}"),
	};
	let _ = cfg
		.define("CMAKE_TOOLCHAIN_FILE", toolchain)
		.define("ANDROID_ABI", abi)
		.define("ANDROID_PLATFORM", ANDROID_PLATFORM);
	// The Visual Studio generator CMake picks on a Windows host can't build for Android.
	if cfg!(windows) && env::var_os("CMAKE_GENERATOR").is_none() {
		let _ = cfg.generator("Ninja");
	}
}
