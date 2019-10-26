// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct GcInteger(Interface<aravis_sys::ArvGcInteger>);

    match fn {
        get_type => || aravis_sys::arv_gc_integer_get_type(),
    }
}

pub const NONE_GC_INTEGER: Option<&GcInteger> = None;

pub trait GcIntegerExt: 'static {
    fn get_inc(&self) -> Result<i64, glib::Error>;

    fn get_max(&self) -> Result<i64, glib::Error>;

    fn get_min(&self) -> Result<i64, glib::Error>;

    fn get_unit(&self) -> Result<GString, glib::Error>;

    fn get_value(&self) -> Result<i64, glib::Error>;

    fn impose_max(&self, maximum: i64) -> Result<(), glib::Error>;

    fn impose_min(&self, minimum: i64) -> Result<(), glib::Error>;

    fn set_value(&self, value: i64) -> Result<(), glib::Error>;
}

impl<O: IsA<GcInteger>> GcIntegerExt for O {
    fn get_inc(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_integer_get_inc(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_max(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_integer_get_max(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_min(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_integer_get_min(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_unit(&self) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_integer_get_unit(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_value(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_integer_get_value(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn impose_max(&self, maximum: i64) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_integer_impose_max(self.as_ref().to_glib_none().0, maximum, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn impose_min(&self, minimum: i64) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_integer_impose_min(self.as_ref().to_glib_none().0, minimum, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_value(&self, value: i64) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_integer_set_value(self.as_ref().to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for GcInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcInteger")
    }
}
