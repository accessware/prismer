//! Contexts, their configuration, and the backends their registry offers.

use core::ffi::{c_char, c_int, c_void};

use crate::{PrismBackend, PrismBackendId, PrismRegistry};

/// An opaque prism context, holding a backend registry.
#[repr(C)]
#[derive(Debug)]
pub struct PrismContext {
	_opaque: [u8; 0],
}

/// The `PrismConfig` layout version this binding describes.
pub const PRISM_CONFIG_VERSION: u8 = 4;

/// The type of a function invoked when a backend's runtime availability changes.
pub type PrismAvailabilityCallback =
	Option<unsafe extern "C" fn(userdata: *mut c_void, backend: PrismBackendId, name: *const c_char, available: bool)>;

/// The type of a function invoked once the poll thread has established its availability baseline.
pub type PrismAvailabilityBaselineCallback = Option<unsafe extern "C" fn(userdata: *mut c_void)>;

/// A struct containing configuration parameters for Prism or it's back-ends to use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PrismConfig {
	/// The version of this structure. This field MUST NOT be modified.
	pub version: u8,
	/// The registry the created context will be bound to. This MAY be `NULL`, in which case the context uses the global registry. If non-null, it MUST be a registry obtained from `prism_registry_freeze`. This field was added in version 3 of this structure.
	pub registry: *mut PrismRegistry,
	/// A function invoked when a backend's runtime availability changes, or `NULL`. When this field is `NULL`, the context performs no background availability polling and creates no poll thread. When it is non-null, the context runs an internal thread that samples backend availability and invokes this callback on each confirmed transition. The behavior of this callback and the polling model are described in the chapter on background availability enumeration. This field was added in version 3 of this structure.
	pub availability_callback: PrismAvailabilityCallback,
	/// An opaque pointer passed unmodified to `availability_callback` on each invocation. Prism does not interpret or take ownership of this value. It is ignored when `availability_callback` is `NULL`. This field was added in version 3 of this structure.
	pub availability_userdata: *mut c_void,
	/// The base interval, in milliseconds, between availability scans. A value of `0` selects the default of 1000 milliseconds. It is ignored when `availability_callback` is `NULL`. This field was added in version 3 of this structure.
	pub availability_poll_interval_ms: u32,
	/// The number of consecutive agreeing samples required before a change in a backend's availability is confirmed and reported. A value of `0` selects the default of 2. A value of `1` confirms every observed change immediately, without debouncing. It is ignored when `availability_callback` is `NULL`. This field was added in version 3 of this structure.
	pub availability_debounce_samples: u32,
	/// The upper bound, in milliseconds, for adaptive backoff of the sampling interval. While availability is unchanging, the interval doubles from `availability_poll_interval_ms` toward this bound, and returns to the base interval as soon as any change is observed. A value of `0`, or any value not greater than the base interval, disables backoff and holds the interval constant. It is ignored when `availability_callback` is `NULL`. This field was added in version 3 of this structure.
	pub availability_backoff_max_ms: u32,
	/// When `true`, and when the library was built with power-management support, the poll thread is paused automatically when the operating system suspends and resumed when it wakes. When `false`, or on builds and platforms without power-management support, this field has no effect and the application MAY drive pausing itself. Use `prism_availability_auto_power_supported` to determine whether this field is honored. It is ignored when `availability_callback` is `NULL`. This field was added in version 3 of this structure.
	pub availability_auto_power_manage: bool,
	/// A function invoked exactly once per context, on the poll thread, after the first availability scan has completed and before the first invocation of `availability_callback`, or `NULL`. It receives `availability_userdata`. It is ignored when `availability_callback` is `NULL`. This field was added in version 4 of this structure.
	pub availability_baseline_callback: PrismAvailabilityBaselineCallback,
}

unsafe extern "C" {
	/// Creates a new configuration structure which can be passed to `prism_init`.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_config_init() -> PrismConfig;
	/// Creates a new Prism context.
	///
	/// # Safety
	///
	/// `cfg` must be null, or point to a writable, initialized `PrismConfig`.
	pub fn prism_init(cfg: *mut PrismConfig) -> *mut PrismContext;
	/// Destroys a Prism context and releases associated resources.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	/// The context is invalid once this returns.
	pub fn prism_shutdown(ctx: *mut PrismContext);
	/// Pauses the availability poll thread.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_availability_poll_pause(ctx: *mut PrismContext);
	/// Resumes the availability poll thread.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_availability_poll_resume(ctx: *mut PrismContext);
	/// Reports whether this build can pause and resume polling automatically in response to operating-system power transitions.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_availability_auto_power_supported() -> bool;
	/// Returns the number of backends registered in the registry.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_count(ctx: *mut PrismContext) -> usize;
	/// Returns the backend ID at the specified index in the registry.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_id_at(ctx: *mut PrismContext, index: usize) -> PrismBackendId;
	/// Looks up a backend by name and returns its ID.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	/// `name` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	pub fn prism_registry_id(ctx: *mut PrismContext, name: *const c_char) -> PrismBackendId;
	/// Returns the human-readable name of a backend given its ID.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_name(ctx: *mut PrismContext, id: PrismBackendId) -> *const c_char;
	/// Returns the priority value of a backend.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_priority(ctx: *mut PrismContext, id: PrismBackendId) -> c_int;
	/// Checks whether a backend with the given ID exists in the registry.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_exists(ctx: *mut PrismContext, id: PrismBackendId) -> bool;
	/// Retrieves a cached backend instance if one exists.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_get(ctx: *mut PrismContext, id: PrismBackendId) -> *mut PrismBackend;
	/// Creates a new backend instance.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_create(ctx: *mut PrismContext, id: PrismBackendId) -> *mut PrismBackend;
	/// Creates a new instance of the highest-priority backend that successfully initializes.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_create_best(ctx: *mut PrismContext) -> *mut PrismBackend;
	/// Acquires a backend instance, reusing a cached instance if available or creating a new one otherwise.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_acquire(ctx: *mut PrismContext, id: PrismBackendId) -> *mut PrismBackend;
	/// Acquires the highest-priority backend that successfully initializes, reusing a cached instance if available.
	///
	/// # Safety
	///
	/// `ctx` must be a live context from `prism_init` that has not been passed to `prism_shutdown`.
	pub fn prism_registry_acquire_best(ctx: *mut PrismContext) -> *mut PrismBackend;
}
