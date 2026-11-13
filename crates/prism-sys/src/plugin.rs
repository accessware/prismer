//! The ABI between prism and a plugin library.

use core::ffi::{c_char, c_int, c_void};

use crate::{PrismBackendVTable, PrismLogLevel};

/// The plugin ABI generation this binding describes.
pub const PRISM_PLUGIN_ABI_VERSION: u64 = 1;

/// A structure through which a backend supplied by a plugin reaches the facilities Prism makes available to it. Prism provides one to each backend it registers from a plugin, as described under Host services.
#[repr(C)]
#[derive(Debug)]
pub struct PrismPluginServices {
	/// The size of this structure as Prism understands it. A backend MUST consult at most this many bytes and MUST treat any member beyond it as absent. A later generation MAY append members after those defined here.
	pub struct_size: u32,
	/// Reserved for future use. Prism sets this member to zero, and a backend MUST ignore it.
	pub reserved: u32,
	/// A function through which the backend records diagnostics. Its first argument MUST be the services object Prism supplied. Prism records the diagnostic under the name of the backend to which the services object belongs; the backend does not supply a source. This member is never `NULL`.
	pub log: Option<unsafe extern "C" fn(self_: *const Self, level: PrismLogLevel, message: *const c_char)>,
}

/// The structure Prism passes as the `userdata` argument to the `create` member of a backend supplied by a plugin. The context is valid only for the duration of the `create` call; the services object it names is valid for the longer period given under Host services.
#[repr(C)]
#[derive(Debug)]
pub struct PrismPluginInstanceContext {
	/// The size of this structure as Prism understands it. A backend MUST consult at most this many bytes and MUST treat any member beyond it as absent.
	pub struct_size: u32,
	/// Reserved for future use. Prism sets this member to zero, and a backend MUST ignore it.
	pub reserved: u32,
	/// The backend's services object. A backend MAY retain it for the period given under Host services. This member is never `NULL`.
	pub services: *const PrismPluginServices,
	/// The value of the `userdata` member of the descriptor from which the backend was registered.
	pub userdata: *mut c_void,
}

/// A structure describing the loading Prism library, passed to a plugin's entry point.
#[repr(C)]
#[derive(Debug)]
pub struct PrismPluginHost {
	/// The plugin ABI generation this implementation provides, equal to the `PRISM_PLUGIN_ABI_VERSION` against which Prism was compiled. A plugin MAY compare this value against its own requirements and decline to supply backends if it requires a newer host.
	pub abi_version: u64,
	/// The size of this structure as Prism understands it. A plugin MUST consult at most this many bytes and MUST treat any member beyond it as absent.
	pub struct_size: u32,
	/// Reserved for future use. Prism sets this member to zero, and a plugin MUST ignore it.
	pub reserved: u32,
	/// A function through which the plugin MAY record diagnostics during the entry point call. Its first argument MUST be the host descriptor Prism supplied; the plugin does not supply a source, and Prism records the diagnostic under a source that names the library being loaded. This member is never `NULL`. A plugin SHOULD use it to state the reason for declining a host, and MUST NOT retain the pointer beyond the entry point call.
	pub log: Option<unsafe extern "C" fn(self_: *const Self, level: PrismLogLevel, message: *const c_char)>,
}

/// A descriptor supplied by a plugin to describe a single backend.
#[repr(C)]
#[derive(Debug)]
pub struct PrismPluginBackend {
	/// The plugin ABI generation this descriptor was built against. A plugin MUST set this member to the `PRISM_PLUGIN_ABI_VERSION` against which it was compiled. Prism rejects a descriptor whose generation it does not accept, as described under ABI compatibility.
	pub abi_version: u64,
	/// The size of this structure as the plugin understands it. A plugin MUST set this member to `sizeof(PrismPluginBackend)`. Prism consults at most this many bytes.
	pub struct_size: u32,
	/// Reserved for future use. A plugin MUST set this member to zero.
	pub reserved: u32,
	/// The backend's name, as a null-terminated UTF-8 string, subject to the same requirements and consequences as the `name` parameter of `prism_registry_builder_add_backend`. The backend's identifier is derived from it by the hash function described in the chapter on backend identifiers. The string is copied during loading.
	pub name: *const c_char,
	/// The backend's priority. Higher values indicate higher priority. This value MUST be non-negative unless the loading call supplies a priority override, in which case it is ignored.
	pub priority: c_int,
	/// The feature set the backend declares, formed by `ORing` `PRISM_BACKEND_*` feature constants together. It MUST be consistent with the vtable in the sense required of any custom backend.
	pub features: u64,
	/// The vtable implementing the backend, subject to every requirement placed on a vtable by the chapter on custom backends. This member MUST NOT be `NULL`, and its `size` member MUST be set as that chapter requires. The vtable MUST provide a `create` member, as described under Host services. The vtable is copied during loading; the code it names remains valid while the library remains loaded.
	pub vtable: *const PrismBackendVTable,
	/// An opaque pointer that Prism delivers to the backend's `create` member as the `userdata` member of a `PrismPluginInstanceContext` (see `PrismPluginInstanceContext`). It has the meaning the `userdata` parameter of `prism_registry_builder_add_backend` would have, and permits several descriptors that share one vtable to be distinguished at instance creation. This member MAY be `NULL`. Prism does not free it.
	pub userdata: *mut c_void,
	/// An informational version identifying the plugin's own release, distinct from the ABI generation and used for no compatibility decision. The value is three 16-bit components, major, minor, and patch, packed most-significant first, with the least-significant 16 bits reserved and set to zero. A plugin MAY set it to zero if it has no version to report.
	pub plugin_version: u64,
}

/// The entry point a prism plugin shared library exports.
pub type PrismPluginQueryFn =
	Option<unsafe extern "C" fn(host: *const PrismPluginHost, index: usize) -> *const PrismPluginBackend>;
