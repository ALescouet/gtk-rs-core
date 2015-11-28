// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use FontChooser;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct FontChooserWidget(Object<ffi::GtkFontChooserWidget>): Widget, Container, Box, Buildable, FontChooser, Orientable;

    match fn {
        get_type => || ffi::gtk_font_chooser_widget_get_type(),
    }
}

impl FontChooserWidget {
    pub fn new() -> FontChooserWidget {
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_widget_new()).downcast_unchecked()
        }
    }

}
