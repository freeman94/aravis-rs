// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcFloat;
use GcNode;
use GcSwissKnife;
use aravis_sys;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GcSwissKnifeNode(Object<aravis_sys::ArvGcSwissKnifeNode, aravis_sys::ArvGcSwissKnifeNodeClass, GcSwissKnifeNodeClass>) @extends GcSwissKnife, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcFloat;

    match fn {
        get_type => || aravis_sys::arv_gc_swiss_knife_node_get_type(),
    }
}

impl GcSwissKnifeNode {
    pub fn new() -> GcSwissKnifeNode {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_swiss_knife_node_new()).unsafe_cast()
        }
    }
}

impl Default for GcSwissKnifeNode {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_SWISS_KNIFE_NODE: Option<&GcSwissKnifeNode> = None;

impl fmt::Display for GcSwissKnifeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcSwissKnifeNode")
    }
}
