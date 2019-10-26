// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Interface;
use aravis_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct UvInterface(Object<aravis_sys::ArvUvInterface, aravis_sys::ArvUvInterfaceClass, UvInterfaceClass>) @extends Interface;

    match fn {
        get_type => || aravis_sys::arv_uv_interface_get_type(),
    }
}

impl UvInterface {
    pub fn get_instance() -> Option<Interface> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(aravis_sys::arv_uv_interface_get_instance())
        }
    }
}

pub const NONE_UV_INTERFACE: Option<&UvInterface> = None;

impl fmt::Display for UvInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UvInterface")
    }
}
