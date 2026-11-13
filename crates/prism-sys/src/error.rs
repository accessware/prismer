//! Error codes.

use core::ffi::{c_char, c_int};

/// An error code. `PRISM_OK` means success; every other value is a failure.
pub type PrismError = c_int;

/// Channel count was retrieved.
pub const PRISM_OK: PrismError = 0;
/// The backend has not been initialized.
pub const PRISM_ERROR_NOT_INITIALIZED: PrismError = 1;
/// The vtable's `size` member was zero, the declared feature set was inconsistent with the vtable, or the priority value was negative.
pub const PRISM_ERROR_INVALID_PARAM: PrismError = 2;
/// The backend does not support audio format queries.
pub const PRISM_ERROR_NOT_IMPLEMENTED: PrismError = 3;
/// No voices are available for this backend
pub const PRISM_ERROR_NO_VOICES: PrismError = 4;
/// The specified voice was not found
pub const PRISM_ERROR_VOICE_NOT_FOUND: PrismError = 5;
/// Speech synthesis failed.
pub const PRISM_ERROR_SPEAK_FAILURE: PrismError = 6;
/// Memory allocation failed during initialization.
pub const PRISM_ERROR_MEMORY_FAILURE: PrismError = 7;
/// A parameter value exceeded its valid range
pub const PRISM_ERROR_RANGE_OUT_OF_BOUNDS: PrismError = 8;
/// An internal error occurred during initialization.
pub const PRISM_ERROR_INTERNAL: PrismError = 9;
/// No speech is currently playing.
pub const PRISM_ERROR_NOT_SPEAKING: PrismError = 10;
/// Speech is not currently paused.
pub const PRISM_ERROR_NOT_PAUSED: PrismError = 11;
/// Speech is already paused.
pub const PRISM_ERROR_ALREADY_PAUSED: PrismError = 12;
/// `text` contains invalid UTF-8 sequences.
pub const PRISM_ERROR_INVALID_UTF8: PrismError = 13;
/// The builder is spent, or a backend with the same name or the same identifier is already present in the builder.
pub const PRISM_ERROR_INVALID_OPERATION: PrismError = 14;
/// The backend was already initialized.
pub const PRISM_ERROR_ALREADY_INITIALIZED: PrismError = 15;
/// The backend's underlying system component is unavailable.
pub const PRISM_ERROR_BACKEND_NOT_AVAILABLE: PrismError = 16;
/// An unspecified error occurred.
pub const PRISM_ERROR_UNKNOWN: PrismError = 17;
/// The audio format which the underlying engine returned to Prism cannot be understood by Prism, or it's parameters are nonsensical.
pub const PRISM_ERROR_INVALID_AUDIO_FORMAT: PrismError = 18;
/// The backend possesses an internal hard ceiling as to how many instances may be instantiated at any given time, and this limit would be exceeded were another to be initialized.
pub const PRISM_ERROR_INTERNAL_BACKEND_LIMIT_EXCEEDED: PrismError = 19;
/// An error occured when the backend was executing a function which has caused the backend to enter an undefined state. The caller should re-initialize the backend from scratch.
pub const PRISM_ERROR_BACKEND_ENTERED_UNDEFINED_STATE: PrismError = 20;
/// A shared library could not be opened, because no file exists at the given path, it is not a loadable image, it was built for a different architecture, or its initialization code failed
pub const PRISM_ERROR_LIBRARY_LOAD_FAILED: PrismError = 21;
/// A shared library was opened but does not export the plugin entry point
pub const PRISM_ERROR_LIBRARY_INVALID: PrismError = 22;
/// A plugin declined the host, or a backend descriptor declared an ABI generation this build of Prism does not accept
pub const PRISM_ERROR_INCOMPATIBLE_ABI: PrismError = 23;
/// The number of defined error codes. Not itself an error.
pub const PRISM_ERROR_COUNT: PrismError = 24;

unsafe extern "C" {
	/// Returns a human-readable description of an error code.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_error_string(error: PrismError) -> *const c_char;
}
