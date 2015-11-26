// This file was generated by gir (e8e8262) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Frame;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct AspectFrame(Object<ffi::GtkAspectFrame>): Widget, Container, Bin, Frame, Buildable;

    match fn {
        get_type => || ffi::gtk_aspect_frame_get_type(),
    }
}

impl AspectFrame {
    pub fn new(label: Option<&str>, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) -> AspectFrame {
        unsafe {
            Widget::from_glib_none(ffi::gtk_aspect_frame_new(label.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib())).downcast_unchecked()
        }
    }

    pub fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) {
        unsafe {
            ffi::gtk_aspect_frame_set(self.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib());
        }
    }

}
