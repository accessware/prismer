//! Registries, and the builders that make them.

use core::ffi::{c_char, c_int, c_void};

use crate::{PrismBackendId, PrismBackendVTable, PrismError};

/// An opaque, immutable set of backend registrations.
#[repr(C)]
#[derive(Debug)]
pub struct PrismRegistry {
	_opaque: [u8; 0],
}

/// An opaque, mutable collection of backend registrations.
#[repr(C)]
#[derive(Debug)]
pub struct PrismRegistryBuilder {
	_opaque: [u8; 0],
}

unsafe extern "C" {
	/// Creates a new registry builder seeded with the compiled-in backends.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_registry_builder_new() -> *mut PrismRegistryBuilder;
	/// Adds a custom backend to a registry builder.
	///
	/// # Safety
	///
	/// `builder` must be a live builder that has not been freed.
	/// `vtable` must point to an initialized `PrismBackendVTable` whose `size` member is set, valid for the call.
	/// `name` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	/// `out_id` must be null, or a writable pointer to a `PrismBackendId`.
	/// `userdata` is passed through untouched; it must satisfy whatever the paired callback expects and stay alive as long as that callback can run.
	pub fn prism_registry_builder_add_backend(
		builder: *mut PrismRegistryBuilder,
		name: *const c_char,
		priority: c_int,
		features: u64,
		vtable: *const PrismBackendVTable,
		userdata: *mut c_void,
		userdata_free: Option<unsafe extern "C" fn(*mut c_void)>,
		out_id: *mut PrismBackendId,
	) -> PrismError;
	/// Loads a plugin from a shared library and adds each backend it supplies to a registry builder.
	///
	/// # Safety
	///
	/// `builder` must be a live builder that has not been freed.
	/// `path` must be a non-null, NUL-terminated UTF-8 string valid for the call.
	/// `out_count` must be null, or a writable pointer to a `usize`.
	pub fn prism_registry_builder_add_library(
		builder: *mut PrismRegistryBuilder,
		path: *const c_char,
		priority_override: c_int,
		out_count: *mut usize,
	) -> PrismError;
	/// Freezes a builder, producing an immutable registry.
	///
	/// # Safety
	///
	/// `builder` must be a live builder that has not been freed.
	pub fn prism_registry_freeze(builder: *mut PrismRegistryBuilder) -> *mut PrismRegistry;
	/// Releases a registry builder.
	///
	/// # Safety
	///
	/// `builder` must be a live builder that has not been freed.
	/// The builder is invalid once this returns.
	pub fn prism_registry_builder_free(builder: *mut PrismRegistryBuilder);
	/// Increments the reference count of a registry.
	///
	/// # Safety
	///
	/// `registry` must be null, or a live registry the caller holds a reference to.
	pub fn prism_registry_retain(registry: *mut PrismRegistry) -> *mut PrismRegistry;
	/// Decrements the reference count of a registry, finalizing it when the count reaches zero.
	///
	/// # Safety
	///
	/// `registry` must be null, or a live registry the caller holds a reference to.
	/// The caller must not release more references than it holds.
	pub fn prism_registry_release(registry: *mut PrismRegistry);
}
