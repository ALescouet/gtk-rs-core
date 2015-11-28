// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget>): Widget, Container, Box, Buildable, FileChooser, Orientable;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action)).downcast_unchecked()
        }
    }

}
