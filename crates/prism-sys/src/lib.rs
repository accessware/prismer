//! Raw FFI bindings to prism (<https://github.com/ethindp/prism>), a
//! platform-agnostic speech and screen reader output library.
#![no_std]
#![allow(non_camel_case_types)]

mod backend;
mod context;
mod custom_backend;
mod error;
mod features;
mod id;
mod log;
mod plugin;
mod registry;

use core::ffi::c_char;

pub use crate::{
	backend::*, context::*, custom_backend::*, error::*, features::*, id::*, log::*, plugin::*, registry::*,
};

unsafe extern "C" {
	/// Returns the version of the loaded Prism library as an encoded integer.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_version() -> u32;
	/// Returns the version of the loaded Prism library as a human-readable string.
	///
	/// # Safety
	///
	/// Safe to call at any time; this is `unsafe` only because it crosses the FFI boundary.
	pub fn prism_version_string() -> *const c_char;
}
