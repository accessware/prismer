//! Backend identifiers, including those of prism's own backends.

/// A backend's identifier, derived from its name.
pub type PrismBackendId = u64;

/// Invalid/sentinel value (always 0)
pub const PRISM_BACKEND_INVALID: PrismBackendId = 0;
/// Microsoft SAPI (Windows)
pub const PRISM_BACKEND_SAPI: PrismBackendId = 0x1D6D_F724_22CE_EE66;
/// `AVSpeechSynthesizer` (macOS, iOS, tvOS, `WatchOS`, `VisionOS`)
pub const PRISM_BACKEND_AV_SPEECH: PrismBackendId = 0x28E3_4295_7780_5C24;
/// `VoiceOver` screen reader (macOS, `MacCatalyst`, iOS, `WatchOS`, tvOS, `VisionOS`)
pub const PRISM_BACKEND_VOICE_OVER: PrismBackendId = 0xCB48_9796_1A75_4BCB;
/// Speech Dispatcher (Linux/BSD, win32 via Wine)
pub const PRISM_BACKEND_SPEECH_DISPATCHER: PrismBackendId = 0xE3D6_F895_D949_EBFE;
/// NVDA screen reader (Windows)
pub const PRISM_BACKEND_NVDA: PrismBackendId = 0x89CC_19C5_C4AC_1A56;
/// JAWS screen reader (Windows)
pub const PRISM_BACKEND_JAWS: PrismBackendId = 0xAC3D_60E9_BD84_B53E;
/// Windows `OneCore` speech API (Windows 10+)
pub const PRISM_BACKEND_ONE_CORE: PrismBackendId = 0x6797_D32F_0D99_4CB4;
/// Orca screen reader (Linux/BSD, win32 via Wine)
pub const PRISM_BACKEND_ORCA: PrismBackendId = 0x10AA_1FC0_5A17_F96C;
/// Android screen readers (Android)
pub const PRISM_BACKEND_ANDROID_SCREEN_READER: PrismBackendId = 0xD199_C175_AEEC_494B;
/// Android TTS engine (Android)
pub const PRISM_BACKEND_ANDROID_TTS: PrismBackendId = 0xBC17_5831_BFE4_E5CC;
/// Web `SpeechSynthesis` API (web)
pub const PRISM_BACKEND_WEB_SPEECH: PrismBackendId = 0x3572_538D_44D4_4A8F;
/// `UIAutomation` backend (Windows only)
pub const PRISM_BACKEND_UIA: PrismBackendId = 0x6238_F019_DB67_8F8E;
/// Zhengdu Screen Reader (Windows)
pub const PRISM_BACKEND_ZDSR: PrismBackendId = 0x3D93_C56C_9E7F_2A2E;
/// `ZoomText` (Windows)
pub const PRISM_BACKEND_ZOOM_TEXT: PrismBackendId = 0xAE43_9D62_DC7B_1479;
/// `BoyPCReader` (windows only)
pub const PRISM_BACKEND_BOY_PC_READER: PrismBackendId = 0x285A_BA1C_16F3_300F;
/// `PCTalker` (windows only)
pub const PRISM_BACKEND_PC_TALKER: PrismBackendId = 0x344B_9519_62E3_B835;
/// Sense Reader screen reader (Windows)
pub const PRISM_BACKEND_SENSE_READER: PrismBackendId = 0xED47_6089_0B55_C2F2;
/// `SystemAccess` screen reader (windows) (only available if explicitly enabled at build time)
pub const PRISM_BACKEND_SYSTEM_ACCESS: PrismBackendId = 0x8380_F2A3_7B2C_3EB6;
/// `WindowEyes` screen reader (windows) (only available if explicitly enabled at build time)
pub const PRISM_BACKEND_WINDOW_EYES: PrismBackendId = 0x9120_D899_0878_5C13;
/// Spiel (Linux and BSDs only)
pub const PRISM_BACKEND_SPIEL: PrismBackendId = 0x478B_44F1_4AD3_D89C;
/// Microsoft GDK speech synthesis (Windows, Xbox) (only built when the GDK's headers are present)
pub const PRISM_BACKEND_GDK_SPEECH: PrismBackendId = 0xD39D_8723_4941_6B11;
/// Places text on the system clipboard instead of presenting it (only used when requested by identifier or name)
pub const PRISM_BACKEND_CLIPBOARD: PrismBackendId = 0x7056_908D_E8F9_F991;
/// Produces no output, for testing (only used when requested by identifier or name)
pub const PRISM_BACKEND_NO_SPEECH: PrismBackendId = 0xC2C0_EAB0_BF97_A5F0;
