// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use Device;
use FakeCamera;

glib_wrapper! {
	pub struct FakeDevice(Object<aravis_sys::ArvFakeDevice, aravis_sys::ArvFakeDeviceClass, FakeDeviceClass>) @extends Device;

	match fn {
		get_type => || aravis_sys::arv_fake_device_get_type(),
	}
}

impl FakeDevice {
	pub fn new(serial_number: &str) -> FakeDevice {
		assert_initialized_main_thread!();
		unsafe {
			Device::from_glib_full(aravis_sys::arv_fake_device_new(
				serial_number.to_glib_none().0,
			))
			.unsafe_cast()
		}
	}
}

pub const NONE_FAKE_DEVICE: Option<&FakeDevice> = None;

/// Trait containing all `FakeDevice` methods.
///
/// # Implementors
///
/// [`FakeDevice`](struct.FakeDevice.html)
pub trait FakeDeviceExt: 'static {
	fn get_fake_camera(&self) -> Option<FakeCamera>;
}

impl<O: IsA<FakeDevice>> FakeDeviceExt for O {
	fn get_fake_camera(&self) -> Option<FakeCamera> {
		unsafe {
			from_glib_none(aravis_sys::arv_fake_device_get_fake_camera(
				self.as_ref().to_glib_none().0,
			))
		}
	}
}

impl fmt::Display for FakeDevice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "FakeDevice")
	}
}
