// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcInteger;
use GcNode;
use GcRegister;
use GcRegisterNode;
use GcSelector;
use aravis_sys;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GcIntRegNode(Object<aravis_sys::ArvGcIntRegNode, aravis_sys::ArvGcIntRegNodeClass, GcIntRegNodeClass>) @extends GcRegisterNode, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcRegister, GcInteger, GcSelector;

    match fn {
        get_type => || aravis_sys::arv_gc_int_reg_node_get_type(),
    }
}

impl GcIntRegNode {
    pub fn new() -> GcIntRegNode {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_int_reg_node_new()).unsafe_cast()
        }
    }
}

impl Default for GcIntRegNode {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_INT_REG_NODE: Option<&GcIntRegNode> = None;

impl fmt::Display for GcIntRegNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcIntRegNode")
    }
}
