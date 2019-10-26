// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use gio;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct XmlSchema(Object<aravis_sys::ArvXmlSchema, aravis_sys::ArvXmlSchemaClass, XmlSchemaClass>);

    match fn {
        get_type => || aravis_sys::arv_xml_schema_get_type(),
    }
}

impl XmlSchema {
    pub fn new_from_file<P: IsA<gio::File>>(file: &P) -> XmlSchema {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_xml_schema_new_from_file(file.as_ref().to_glib_none().0))
        }
    }

    pub fn new_from_path(path: &str) -> XmlSchema {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_xml_schema_new_from_path(path.to_glib_none().0))
        }
    }
}

pub const NONE_XML_SCHEMA: Option<&XmlSchema> = None;

pub trait XmlSchemaExt: 'static {
    //fn validate(&self, xml: /*Unimplemented*/Option<Fundamental: Pointer>, size: usize, line: i32, column: i32) -> Result<(), glib::Error>;
}

impl<O: IsA<XmlSchema>> XmlSchemaExt for O {
    //fn validate(&self, xml: /*Unimplemented*/Option<Fundamental: Pointer>, size: usize, line: i32, column: i32) -> Result<(), glib::Error> {
    //    unsafe { TODO: call aravis_sys:arv_xml_schema_validate() }
    //}
}

impl fmt::Display for XmlSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XmlSchema")
    }
}
