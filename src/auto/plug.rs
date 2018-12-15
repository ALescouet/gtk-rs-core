// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Component;
use Object;
use ffi;
use glib::GString;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Plug(Object<ffi::AtkPlug, ffi::AtkPlugClass>): Object, Component;

    match fn {
        get_type => || ffi::atk_plug_get_type(),
    }
}

impl Plug {
    pub fn new() -> Plug {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(ffi::atk_plug_new()).downcast_unchecked()
        }
    }
}

impl Default for Plug {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AtkPlugExt: 'static {
    fn get_id(&self) -> Option<GString>;
}

impl<O: IsA<Plug>> AtkPlugExt for O {
    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::atk_plug_get_id(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for Plug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plug")
    }
}
