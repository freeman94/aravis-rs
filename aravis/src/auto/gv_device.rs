// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Device;
use GvStreamOption;
use aravis_sys;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GvDevice(Object<aravis_sys::ArvGvDevice, aravis_sys::ArvGvDeviceClass, GvDeviceClass>) @extends Device;

    match fn {
        get_type => || aravis_sys::arv_gv_device_get_type(),
    }
}

impl GvDevice {
    pub fn new<P: IsA<gio::InetAddress>, Q: IsA<gio::InetAddress>>(interface_address: &P, device_address: &Q) -> GvDevice {
        assert_initialized_main_thread!();
        unsafe {
            Device::from_glib_full(aravis_sys::arv_gv_device_new(interface_address.as_ref().to_glib_none().0, device_address.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    //pub fn get_url_regex() -> /*Ignored*/Option<glib::Regex> {
    //    unsafe { TODO: call aravis_sys:arv_gv_device_get_url_regex() }
    //}
}

pub const NONE_GV_DEVICE: Option<&GvDevice> = None;

pub trait GvDeviceExt: 'static {
    fn auto_packet_size(&self) -> u32;

    fn get_device_address(&self) -> Option<gio::SocketAddress>;

    fn get_interface_address(&self) -> Option<gio::SocketAddress>;

    fn get_packet_size(&self) -> u32;

    fn get_stream_options(&self) -> GvStreamOption;

    fn get_timestamp_tick_frequency(&self) -> u64;

    fn set_packet_size(&self, packet_size: i32);

    fn set_stream_options(&self, options: GvStreamOption);
}

impl<O: IsA<GvDevice>> GvDeviceExt for O {
    fn auto_packet_size(&self) -> u32 {
        unsafe {
            aravis_sys::arv_gv_device_auto_packet_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_device_address(&self) -> Option<gio::SocketAddress> {
        unsafe {
            from_glib_none(aravis_sys::arv_gv_device_get_device_address(self.as_ref().to_glib_none().0))
        }
    }

    fn get_interface_address(&self) -> Option<gio::SocketAddress> {
        unsafe {
            from_glib_none(aravis_sys::arv_gv_device_get_interface_address(self.as_ref().to_glib_none().0))
        }
    }

    fn get_packet_size(&self) -> u32 {
        unsafe {
            aravis_sys::arv_gv_device_get_packet_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_stream_options(&self) -> GvStreamOption {
        unsafe {
            from_glib(aravis_sys::arv_gv_device_get_stream_options(self.as_ref().to_glib_none().0))
        }
    }

    fn get_timestamp_tick_frequency(&self) -> u64 {
        unsafe {
            aravis_sys::arv_gv_device_get_timestamp_tick_frequency(self.as_ref().to_glib_none().0)
        }
    }

    fn set_packet_size(&self, packet_size: i32) {
        unsafe {
            aravis_sys::arv_gv_device_set_packet_size(self.as_ref().to_glib_none().0, packet_size);
        }
    }

    fn set_stream_options(&self, options: GvStreamOption) {
        unsafe {
            aravis_sys::arv_gv_device_set_stream_options(self.as_ref().to_glib_none().0, options.to_glib());
        }
    }
}

impl fmt::Display for GvDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GvDevice")
    }
}
