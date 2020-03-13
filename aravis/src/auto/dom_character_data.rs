// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use DomNode;

glib_wrapper! {
	pub struct DomCharacterData(Object<aravis_sys::ArvDomCharacterData, aravis_sys::ArvDomCharacterDataClass, DomCharacterDataClass>) @extends DomNode;

	match fn {
		get_type => || aravis_sys::arv_dom_character_data_get_type(),
	}
}

pub const NONE_DOM_CHARACTER_DATA: Option<&DomCharacterData> = None;

/// Trait containing all `DomCharacterData` methods.
///
/// # Implementors
///
/// [`DomCharacterData`](struct.DomCharacterData.html), [`DomText`](struct.DomText.html)
pub trait DomCharacterDataExt: 'static {
	fn get_data(&self) -> Option<GString>;

	fn set_data(&self, value: &str);
}

impl<O: IsA<DomCharacterData>> DomCharacterDataExt for O {
	fn get_data(&self) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_dom_character_data_get_data(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn set_data(&self, value: &str) {
		unsafe {
			aravis_sys::arv_dom_character_data_set_data(
				self.as_ref().to_glib_none().0,
				value.to_glib_none().0,
			);
		}
	}
}

impl fmt::Display for DomCharacterData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DomCharacterData")
	}
}
