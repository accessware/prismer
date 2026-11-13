//! The vtable a custom backend implements.

use core::ffi::{c_char, c_void};

use crate::{PrismAudioCallback, PrismError};

/// A table of function pointers implementing a custom backend.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PrismBackendVTable {
	/// The size of this structure as known to the application. This member MUST be set to `sizeof(PrismBackendVTable)`. Prism reads at most `size` bytes from the structure. If `size` is smaller than the size of the structure as this version of Prism defines it, the members beyond `size` are treated as null; if it is larger, the additional bytes are ignored. This scheme permits the structure to grow in later library versions without invalidating applications compiled against earlier ones.
	pub size: usize,
	/// An optional function producing a per-instance state pointer. If non-null, Prism invokes it exactly once for each backend instance constructed from the registration, passing the registration's `userdata`, and thereafter passes the returned pointer as the `instance` argument to every other member invoked for that instance. Should `create` return `NULL`, construction of the instance fails. If `create` is null, the registration's `userdata` pointer is passed as the `instance` argument directly, and all instances of the backend consequently share it.
	pub create: Option<unsafe extern "C" fn(userdata: *mut c_void) -> *mut c_void>,
	/// An optional function releasing a state pointer previously returned by `create`. If both `create` and `destroy` are non-null, Prism invokes `destroy` exactly once for each backend instance, at the time the instance is freed. `destroy` is never invoked if `create` is null.
	pub destroy: Option<unsafe extern "C" fn(instance: *mut c_void)>,
	/// An optional runtime availability probe. If non-null, Prism invokes it to determine the `PRISM_BACKEND_IS_SUPPORTED_AT_RUNTIME` bit reported by `prism_backend_get_features`; if null, the bit declared at registration is reported unchanged. Because `prism_backend_get_features` MAY be called before initialization, `is_supported` MAY be invoked before `initialize` has succeeded, and an implementation of it MUST NOT assume the instance has been initialized. This member designates no operation and is therefore exempt from the feature consistency requirement of `prism_registry_builder_add_backend`.
	pub is_supported: Option<unsafe extern "C" fn(instance: *mut c_void) -> bool>,
	/// Implements `prism_backend_initialize`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub initialize: Option<unsafe extern "C" fn(instance: *mut c_void) -> PrismError>,
	/// Implements `prism_backend_speak`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub speak: Option<unsafe extern "C" fn(instance: *mut c_void, text: *const c_char, interrupt: bool) -> PrismError>,
	/// Implements `prism_backend_speak_to_memory`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub speak_to_memory: Option<
		unsafe extern "C" fn(
			instance: *mut c_void,
			text: *const c_char,
			callback: PrismAudioCallback,
			callback_userdata: *mut c_void,
		) -> PrismError,
	>,
	/// Implements `prism_backend_braille`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub braille: Option<unsafe extern "C" fn(instance: *mut c_void, text: *const c_char) -> PrismError>,
	/// Implements `prism_backend_output`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub output: Option<unsafe extern "C" fn(instance: *mut c_void, text: *const c_char, interrupt: bool) -> PrismError>,
	/// Implements `prism_backend_stop`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub stop: Option<unsafe extern "C" fn(instance: *mut c_void) -> PrismError>,
	/// Implements `prism_backend_pause`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub pause: Option<unsafe extern "C" fn(instance: *mut c_void) -> PrismError>,
	/// Implements `prism_backend_resume`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub resume: Option<unsafe extern "C" fn(instance: *mut c_void) -> PrismError>,
	/// Implements `prism_backend_is_speaking`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub is_speaking: Option<unsafe extern "C" fn(instance: *mut c_void, out_speaking: *mut bool) -> PrismError>,
	/// Implements `prism_backend_set_volume`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub set_volume: Option<unsafe extern "C" fn(instance: *mut c_void, volume: f32) -> PrismError>,
	/// Implements `prism_backend_get_volume`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_volume: Option<unsafe extern "C" fn(instance: *mut c_void, out_volume: *mut f32) -> PrismError>,
	/// Implements `prism_backend_set_rate`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub set_rate: Option<unsafe extern "C" fn(instance: *mut c_void, rate: f32) -> PrismError>,
	/// Implements `prism_backend_get_rate`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_rate: Option<unsafe extern "C" fn(instance: *mut c_void, out_rate: *mut f32) -> PrismError>,
	/// Implements `prism_backend_set_pitch`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub set_pitch: Option<unsafe extern "C" fn(instance: *mut c_void, pitch: f32) -> PrismError>,
	/// Implements `prism_backend_get_pitch`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_pitch: Option<unsafe extern "C" fn(instance: *mut c_void, out_pitch: *mut f32) -> PrismError>,
	/// Implements `prism_backend_refresh_voices`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub refresh_voices: Option<unsafe extern "C" fn(instance: *mut c_void) -> PrismError>,
	/// Implements `prism_backend_count_voices`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub count_voices: Option<unsafe extern "C" fn(instance: *mut c_void, out_count: *mut usize) -> PrismError>,
	/// Implements `prism_backend_get_voice_name`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_voice_name: Option<
		unsafe extern "C" fn(instance: *mut c_void, voice_id: usize, out_name: *mut *const c_char) -> PrismError,
	>,
	/// Implements `prism_backend_get_voice_language`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_voice_language: Option<
		unsafe extern "C" fn(instance: *mut c_void, voice_id: usize, out_language: *mut *const c_char) -> PrismError,
	>,
	/// Implements `prism_backend_set_voice`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub set_voice: Option<unsafe extern "C" fn(instance: *mut c_void, voice_id: usize) -> PrismError>,
	/// Implements `prism_backend_get_voice`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_voice: Option<unsafe extern "C" fn(instance: *mut c_void, out_voice_id: *mut usize) -> PrismError>,
	/// Implements `prism_backend_get_channels`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_channels: Option<unsafe extern "C" fn(instance: *mut c_void, out_channels: *mut usize) -> PrismError>,
	/// Implements `prism_backend_get_sample_rate`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_sample_rate: Option<unsafe extern "C" fn(instance: *mut c_void, out_sample_rate: *mut usize) -> PrismError>,
	/// Implements `prism_backend_get_bit_depth`, taking the instance pointer in place of the backend. Null means the operation is unimplemented.
	pub get_bit_depth: Option<unsafe extern "C" fn(instance: *mut c_void, out_bit_depth: *mut usize) -> PrismError>,
}
