use crate::{Device, Stream};

use glib::translate::{from_glib, from_glib_full, ToGlibPtr};
use glib::IsA;
use std::os::raw::c_void;

pub(crate) mod traits {
	use super::*;

	/// Trait containing additional [`Device`] methods.
	pub trait DeviceExtManual {
		fn create_stream(&self) -> Result<Stream, glib::Error>;
		fn write_memory(&self, address: u64, buffer: &mut [u8]) -> Result<bool, glib::Error>;
		fn read_memory(&self, address: u64, buffer: &mut [u8]) -> Result<bool, glib::Error>;
	}
}

impl<T: IsA<Device>> traits::DeviceExtManual for T {
	fn create_stream(&self) -> Result<Stream, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let stream = aravis_sys::arv_device_create_stream(
				self.as_ref().to_glib_none().0,
				None,
				std::ptr::null_mut(),
				&mut error,
			);
			if error.is_null() {
				Ok(glib::translate::from_glib_full(stream))
			} else {
				Err(glib::translate::from_glib_full(error))
			}
		}
	}

	// `buffer` is not marked const in aravis, hence the need for mutability here.
	fn write_memory(&self, address: u64, buffer: &mut [u8]) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = aravis_sys::arv_device_write_memory(
				self.as_ref().to_glib_none().0,
				address,
				buffer.len() as u32,
				buffer.as_mut_ptr() as *mut c_void,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn read_memory(&self, address: u64, buffer: &mut [u8]) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = aravis_sys::arv_device_read_memory(
				self.as_ref().to_glib_none().0,
				address,
				buffer.len() as u32,
				buffer.as_mut_ptr() as *mut c_void,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}
