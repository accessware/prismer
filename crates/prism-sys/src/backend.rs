//! Backend instances and the operations on them.

use core::ffi::{c_char, c_void};

use crate::PrismError;

/// An opaque backend instance obtained from a registry.
#[repr(C)]
#[derive(Debug)]
pub struct PrismBackend {
	_opaque: [u8; 0],
}

/// Receives audio samples from `prism_backend_speak_to_memory`.
pub type PrismAudioCallback = Option<
	unsafe extern "C" fn(
		userdata: *mut c_void,
		samples: *const f32,
		sample_count: usize,
		channels: usize,
		sample_rate: usize,
	),
>;

unsafe extern "C" {
	/// Releases a backend instance.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// The pointer is invalid once this returns.
	pub fn prism_backend_free(backend: *mut PrismBackend);
	/// Returns the human-readable name of a backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_name(backend: *mut PrismBackend) -> *const c_char;
	/// Returns a bitmask of all features supported by this backend, as well as other information.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_get_features(backend: *mut PrismBackend) -> u64;
	/// Initializes a backend instance.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_initialize(backend: *mut PrismBackend) -> PrismError;
	/// Synthesizes speech from the given text and plays it through the default audio output.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `text` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	pub fn prism_backend_speak(backend: *mut PrismBackend, text: *const c_char, interrupt: bool) -> PrismError;
	/// Synthesizes speech from the given text and delivers the audio samples to a callback function.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `text` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	/// `userdata` is passed through untouched; it must satisfy whatever the paired callback expects and stay alive as long as that callback can run.
	pub fn prism_backend_speak_to_memory(
		backend: *mut PrismBackend,
		text: *const c_char,
		callback: PrismAudioCallback,
		userdata: *mut c_void,
	) -> PrismError;
	/// Outputs text to a connected braille display.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `text` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	pub fn prism_backend_braille(backend: *mut PrismBackend, text: *const c_char) -> PrismError;
	/// Outputs text using all available modalities supported by the backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `text` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	pub fn prism_backend_output(backend: *mut PrismBackend, text: *const c_char, interrupt: bool) -> PrismError;
	/// Immediately stops any currently playing speech.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_stop(backend: *mut PrismBackend) -> PrismError;
	/// Pauses currently playing speech.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_pause(backend: *mut PrismBackend) -> PrismError;
	/// Resumes previously paused speech.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_resume(backend: *mut PrismBackend) -> PrismError;
	/// Queries whether the backend is currently producing speech output.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_speaking` must be null, or a writable pointer to a `bool`.
	pub fn prism_backend_is_speaking(backend: *mut PrismBackend, out_speaking: *mut bool) -> PrismError;
	/// Sets the speech volume.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_set_volume(backend: *mut PrismBackend, volume: f32) -> PrismError;
	/// Retrieves the current speech volume.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_volume` must be null, or a writable pointer to a `f32`.
	pub fn prism_backend_get_volume(backend: *mut PrismBackend, out_volume: *mut f32) -> PrismError;
	/// Sets the speech rate (speed).
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_set_rate(backend: *mut PrismBackend, rate: f32) -> PrismError;
	/// Retrieves the current speech rate.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_rate` must be null, or a writable pointer to a `f32`.
	pub fn prism_backend_get_rate(backend: *mut PrismBackend, out_rate: *mut f32) -> PrismError;
	/// Sets the speech pitch.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_set_pitch(backend: *mut PrismBackend, pitch: f32) -> PrismError;
	/// Retrieves the current speech pitch.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_pitch` must be null, or a writable pointer to a `f32`.
	pub fn prism_backend_get_pitch(backend: *mut PrismBackend, out_pitch: *mut f32) -> PrismError;
	/// Refreshes the backend's internal voice list.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_refresh_voices(backend: *mut PrismBackend) -> PrismError;
	/// Returns the number of voices available from the backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_count` must be null, or a writable pointer to a `usize`.
	pub fn prism_backend_count_voices(backend: *mut PrismBackend, out_count: *mut usize) -> PrismError;
	/// Retrieves the human-readable name of a voice.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_name` must be null, or a writable pointer to a `*const c_char`.
	pub fn prism_backend_get_voice_name(
		backend: *mut PrismBackend,
		voice_id: usize,
		out_name: *mut *const c_char,
	) -> PrismError;
	/// Retrieves the language code or language string of a voice.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_language` must be null, or a writable pointer to a `*const c_char`.
	pub fn prism_backend_get_voice_language(
		backend: *mut PrismBackend,
		voice_id: usize,
		out_language: *mut *const c_char,
	) -> PrismError;
	/// Selects a voice to use for subsequent speech synthesis.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	pub fn prism_backend_set_voice(backend: *mut PrismBackend, voice_id: usize) -> PrismError;
	/// Retrieves the index of the currently selected voice.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_voice_id` must be null, or a writable pointer to a `usize`.
	pub fn prism_backend_get_voice(backend: *mut PrismBackend, out_voice_id: *mut usize) -> PrismError;
	/// Retrieves the number of audio channels produced by the backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_channels` must be null, or a writable pointer to a `usize`.
	pub fn prism_backend_get_channels(backend: *mut PrismBackend, out_channels: *mut usize) -> PrismError;
	/// Retrieves the sample rate of audio produced by the backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_sample_rate` must be null, or a writable pointer to a `usize`.
	pub fn prism_backend_get_sample_rate(backend: *mut PrismBackend, out_sample_rate: *mut usize) -> PrismError;
	/// Retrieves the native bit depth of audio produced by the backend.
	///
	/// # Safety
	///
	/// `backend` must be a live backend that has not been passed to `prism_backend_free`.
	/// `out_bit_depth` must be null, or a writable pointer to a `usize`.
	pub fn prism_backend_get_bit_depth(backend: *mut PrismBackend, out_bit_depth: *mut usize) -> PrismError;
}
