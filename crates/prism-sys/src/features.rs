//! The feature flags `prism_backend_get_features` reports.

/// The underlying engine or service is available. This determination is advisory; `prism_backend_initialize` MAY still fail.
pub const PRISM_BACKEND_IS_SUPPORTED_AT_RUNTIME: u64 = 1 << 0;
/// `prism_backend_speak` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SPEAK: u64 = 1 << 2;
/// `prism_backend_speak_to_memory` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SPEAK_TO_MEMORY: u64 = 1 << 3;
/// `prism_backend_braille` is implemented.
pub const PRISM_BACKEND_SUPPORTS_BRAILLE: u64 = 1 << 4;
/// `prism_backend_output` is implemented.
pub const PRISM_BACKEND_SUPPORTS_OUTPUT: u64 = 1 << 5;
/// `prism_backend_is_speaking` is implemented.
pub const PRISM_BACKEND_SUPPORTS_IS_SPEAKING: u64 = 1 << 6;
/// `prism_backend_stop` is implemented.
pub const PRISM_BACKEND_SUPPORTS_STOP: u64 = 1 << 7;
/// `prism_backend_pause` is implemented.
pub const PRISM_BACKEND_SUPPORTS_PAUSE: u64 = 1 << 8;
/// `prism_backend_resume` is implemented.
pub const PRISM_BACKEND_SUPPORTS_RESUME: u64 = 1 << 9;
/// `prism_backend_set_volume` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SET_VOLUME: u64 = 1 << 10;
/// `prism_backend_get_volume` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_VOLUME: u64 = 1 << 11;
/// `prism_backend_set_rate` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SET_RATE: u64 = 1 << 12;
/// `prism_backend_get_rate` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_RATE: u64 = 1 << 13;
/// `prism_backend_set_pitch` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SET_PITCH: u64 = 1 << 14;
/// `prism_backend_get_pitch` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_PITCH: u64 = 1 << 15;
/// `prism_backend_refresh_voices` is implemented.
pub const PRISM_BACKEND_SUPPORTS_REFRESH_VOICES: u64 = 1 << 16;
/// `prism_backend_count_voices` is implemented.
pub const PRISM_BACKEND_SUPPORTS_COUNT_VOICES: u64 = 1 << 17;
/// `prism_backend_get_voice_name` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_VOICE_NAME: u64 = 1 << 18;
/// `prism_backend_get_voice_language` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_VOICE_LANGUAGE: u64 = 1 << 19;
/// `prism_backend_get_voice` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_VOICE: u64 = 1 << 20;
/// `prism_backend_set_voice` is implemented.
pub const PRISM_BACKEND_SUPPORTS_SET_VOICE: u64 = 1 << 21;
/// `prism_backend_get_channels` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_CHANNELS: u64 = 1 << 22;
/// `prism_backend_get_sample_rate` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_SAMPLE_RATE: u64 = 1 << 23;
/// `prism_backend_get_bit_depth` is implemented.
pub const PRISM_BACKEND_SUPPORTS_GET_BIT_DEPTH: u64 = 1 << 24;
/// Reserved.
pub const PRISM_BACKEND_PERFORMS_SILENCE_TRIMMING_ON_SPEAK: u64 = 1 << 25;
/// The backend trims leading and trailing silence from the audio stream before delivering it to the audio callback.
pub const PRISM_BACKEND_PERFORMS_SILENCE_TRIMMING_ON_SPEAK_TO_MEMORY: u64 = 1 << 26;
/// Reserved.
pub const PRISM_BACKEND_SUPPORTS_SPEAK_SSML: u64 = 1 << 27;
/// Reserved.
pub const PRISM_BACKEND_SUPPORTS_SPEAK_TO_MEMORY_SSML: u64 = 1 << 28;
/// The backend presents text through a screen reader.
pub const PRISM_BACKEND_IS_SCREEN_READER: u64 = 1 << 56;
/// The backend synthesizes speech without a screen reader.
pub const PRISM_BACKEND_IS_SPEECH_ENGINE: u64 = 1 << 57;
/// The backend is never selected by `prism_registry_create_best` or `prism_registry_acquire_best`, and is obtained only by requesting it by identifier or name.
pub const PRISM_BACKEND_REQUIRES_EXPLICIT_SELECTION: u64 = 1 << 58;
/// The highest bit reserved for feature flags.
pub const PRISM_BACKEND_FEATURE_MAX_BIT: u64 = 1 << 63;
