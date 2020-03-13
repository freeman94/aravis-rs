// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;
use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;

glib_wrapper! {
	pub struct GcCommand(Object<aravis_sys::ArvGcCommand, aravis_sys::ArvGcCommandClass, GcCommandClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		get_type => || aravis_sys::arv_gc_command_get_type(),
	}
}

impl GcCommand {
	pub fn new() -> GcCommand {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(aravis_sys::arv_gc_command_new()).unsafe_cast() }
	}
}

impl Default for GcCommand {
	fn default() -> Self {
		Self::new()
	}
}

pub const NONE_GC_COMMAND: Option<&GcCommand> = None;

/// Trait containing all `GcCommand` methods.
///
/// # Implementors
///
/// [`GcCommand`](struct.GcCommand.html)
pub trait GcCommandExt: 'static {
	fn execute(&self) -> Result<(), glib::Error>;
}

impl<O: IsA<GcCommand>> GcCommandExt for O {
	fn execute(&self) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_gc_command_execute(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl fmt::Display for GcCommand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcCommand")
	}
}
