// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EmblemOrigin, Icon};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GEmblem")]
    pub struct Emblem(Object<ffi::GEmblem, ffi::GEmblemClass>) @implements Icon;

    match fn {
        type_ => || ffi::g_emblem_get_type(),
    }
}

impl Emblem {
    #[doc(alias = "g_emblem_new")]
    pub fn new(icon: &impl IsA<Icon>) -> Emblem {
        unsafe { from_glib_full(ffi::g_emblem_new(icon.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_emblem_new_with_origin")]
    #[doc(alias = "new_with_origin")]
    pub fn with_origin(icon: &impl IsA<Icon>, origin: EmblemOrigin) -> Emblem {
        unsafe {
            from_glib_full(ffi::g_emblem_new_with_origin(
                icon.as_ref().to_glib_none().0,
                origin.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_emblem_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self) -> Icon {
        unsafe { from_glib_none(ffi::g_emblem_get_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_emblem_get_origin")]
    #[doc(alias = "get_origin")]
    pub fn origin(&self) -> EmblemOrigin {
        unsafe { from_glib(ffi::g_emblem_get_origin(self.to_glib_none().0)) }
    }
}
