// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Adjustment;
use Bin;
use Buildable;
use Button;
use Container;
use Orientable;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ScaleButton(Object<ffi::GtkScaleButton>): Widget, Container, Bin, Button, Actionable, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    //pub fn new(size: i32, min: f64, max: f64, step: f64, icons: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") -> ScaleButton {
    //    unsafe { TODO: call ffi::gtk_scale_button_new() }
    //}

}

pub trait ScaleButtonExt {
    fn get_adjustment(&self) -> Option<Adjustment>;
    fn get_minus_button(&self) -> Option<Widget>;
    fn get_plus_button(&self) -> Option<Widget>;
    fn get_popup(&self) -> Option<Widget>;
    fn get_value(&self) -> f64;
    fn set_adjustment(&self, adjustment: &Adjustment);
    //fn set_icons(&self, icons: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }");
    fn set_value(&self, value: f64);
}

impl<O: Upcast<ScaleButton>> ScaleButtonExt for O {
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(self.to_glib_none().0))
        }
    }

    fn get_minus_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_minus_button(self.to_glib_none().0))
        }
    }

    fn get_plus_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_plus_button(self.to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_popup(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(self.to_glib_none().0)
        }
    }

    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    //fn set_icons(&self, icons: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") {
    //    unsafe { TODO: call ffi::gtk_scale_button_set_icons() }
    //}

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_scale_button_set_value(self.to_glib_none().0, value);
        }
    }

}
