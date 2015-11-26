// This file was generated by gir (e8e8262) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use Buildable;
use ButtonBoxStyle;
use Container;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ButtonBox(Object<ffi::GtkButtonBox>): Widget, Container, Box, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_button_box_get_type(),
    }
}

impl ButtonBox {
    pub fn new(orientation: Orientation) -> ButtonBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation)).downcast_unchecked()
        }
    }

    pub fn get_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_non_homogeneous(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_child_secondary<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_secondary(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            ffi::gtk_button_box_get_layout(self.to_glib_none().0)
        }
    }

    pub fn set_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(self.to_glib_none().0, child.to_glib_none().0, non_homogeneous.to_glib());
        }
    }

    pub fn set_child_secondary<T: Upcast<Widget>>(&self, child: &T, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(self.to_glib_none().0, child.to_glib_none().0, is_secondary.to_glib());
        }
    }

    pub fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(self.to_glib_none().0, layout_style);
        }
    }

}
