// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Action;
use Component;
use Document;
use EditableText;
use Hypertext;
use Image;
use Object;
use Selection;
use Table;
use TableCell;
use Text;
use Value;
use Window;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct NoOpObject(Object<ffi::AtkNoOpObject, ffi::AtkNoOpObjectClass>): Object, Action, Component, Document, EditableText, Hypertext, Image, Selection, Table, TableCell, Text, Value, Window;

    match fn {
        get_type => || ffi::atk_no_op_object_get_type(),
    }
}

impl NoOpObject {
    pub fn new<P: IsA<glib::Object>>(obj: &P) -> NoOpObject {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(ffi::atk_no_op_object_new(obj.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl fmt::Display for NoOpObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoOpObject")
    }
}
