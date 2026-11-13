//! The Rust half of prismer's Android example. `MainActivity` loads it and calls `speak`, which
//! logs what it does to logcat under the `PRISMER` tag.

use std::{
	ffi::{CString, c_char, c_int, c_void},
	thread,
	time::Duration,
};

use prismer::{Features, Prism};

/// logcat priorities, from `android/log.h`.
const INFO: c_int = 4;
const ERROR: c_int = 6;

#[link(name = "log")]
unsafe extern "C" {
	fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

fn log(priority: c_int, text: &str) {
	let text = CString::new(text.replace('\0', "")).unwrap_or_default();
	// SAFETY: both strings are NUL terminated and outlive the call.
	unsafe { __android_log_write(priority, c"PRISMER".as_ptr(), text.as_ptr()) };
}

fn speak() -> prismer::Result<()> {
	let prism = Prism::new()?;
	log(INFO, &format!("prism {}", prismer::version_string()));
	for id in prism.backend_ids() {
		log(INFO, &format!("backend {}", prism.backend_name(id).unwrap_or_default()));
	}
	let backend = prism.create_best()?;
	log(INFO, &format!("best {}", backend.name()));
	backend.speak("Hello from prismer on Android.", false)?;
	log(INFO, "spoke");
	// Dropping the backend stops its speech, so it's kept until it finishes.
	if backend.supports(Features::IS_SPEAKING) {
		while backend.is_speaking()? {
			thread::sleep(Duration::from_millis(50));
		}
	} else {
		thread::sleep(Duration::from_secs(3));
	}
	log(INFO, "done");
	Ok(())
}

/// `MainActivity.speak`. Starting Android's speech engine waits on a callback that arrives on the
/// main thread, so `MainActivity` calls this from another one.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_accessware_prismer_example_MainActivity_speak(_env: *mut c_void, _class: *mut c_void) {
	if let Err(error) = speak() {
		log(ERROR, &format!("failed: {error}"));
	}
}
