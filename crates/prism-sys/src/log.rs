//! prism's process-wide logger.

use core::ffi::{c_char, c_int, c_void};

/// The severity of a log message, or a threshold for delivering them.
pub type PrismLogLevel = c_int;

/// The most verbose level, used for fine-grained tracing of internal operations.
pub const PRISM_LOG_LEVEL_TRACE: PrismLogLevel = 0;
/// Diagnostic information useful during development.
pub const PRISM_LOG_LEVEL_DEBUG: PrismLogLevel = 1;
/// Informational messages describing normal operation.
pub const PRISM_LOG_LEVEL_INFO: PrismLogLevel = 2;
/// Conditions that are not errors but MAY indicate a problem.
pub const PRISM_LOG_LEVEL_WARN: PrismLogLevel = 3;
/// Error conditions.
pub const PRISM_LOG_LEVEL_ERROR: PrismLogLevel = 4;
/// Not a message severity. When supplied to `prism_set_log_level`, it suppresses all messages, since no message has a severity greater than or equal to it.
pub const PRISM_LOG_LEVEL_NONE: PrismLogLevel = 5;

/// The type of a function invoked to deliver a single log message.
pub type PrismLogCallback = Option<
	unsafe extern "C" fn(userdata: *mut c_void, level: PrismLogLevel, source: *const c_char, message: *const c_char),
>;

/// A structure pairing a log callback with an opaque user pointer.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PrismLogHandler {
	/// The callback invoked to deliver messages, or `NULL` to install no handler. When `fn` is `NULL`, messages are discarded.
	pub fn_: PrismLogCallback,
	/// An opaque pointer passed unmodified to `fn` on each invocation. Prism does not interpret or take ownership of this value. The lifetime of this value is the lifetime of the handler function, and therefore this value must be valid for as long as the handler is alive.
	pub userdata: *mut c_void,
}

unsafe extern "C" {
	/// Installs the handler that receives log messages, replacing any previously installed handler.
	///
	/// # Safety
	///
	/// `handler.fn_`, if set, must be safe to call with `handler.userdata` from prism's logging thread, and that userdata must outlive the handler.
	pub fn prism_set_log_handler(handler: PrismLogHandler) -> PrismLogHandler;
	/// Sets the minimum severity of messages that will be delivered.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_set_log_level(level: PrismLogLevel) -> PrismLogLevel;
	/// Emits a log message.
	///
	/// # Safety
	///
	/// `source` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	/// `message` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	pub fn prism_log(level: PrismLogLevel, source: *const c_char, message: *const c_char);
	/// Blocks until all messages queued before the call have been delivered.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_log_flush();
	/// Stops the logging thread and releases the resources associated with the logger.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_log_shutdown();
}
