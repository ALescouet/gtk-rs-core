// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, TestDBusFlags};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GTestDBus")]
    pub struct TestDBus(Object<ffi::GTestDBus>);

    match fn {
        type_ => || ffi::g_test_dbus_get_type(),
    }
}

impl TestDBus {
    #[doc(alias = "g_test_dbus_new")]
    pub fn new(flags: TestDBusFlags) -> TestDBus {
        unsafe { from_glib_full(ffi::g_test_dbus_new(flags.into_glib())) }
    }

    #[doc(alias = "g_test_dbus_add_service_dir")]
    pub fn add_service_dir(&self, path: &str) {
        unsafe {
            ffi::g_test_dbus_add_service_dir(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "g_test_dbus_down")]
    pub fn down(&self) {
        unsafe {
            ffi::g_test_dbus_down(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_test_dbus_get_bus_address")]
    #[doc(alias = "get_bus_address")]
    pub fn bus_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_test_dbus_get_bus_address(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_test_dbus_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> TestDBusFlags {
        unsafe { from_glib(ffi::g_test_dbus_get_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_test_dbus_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::g_test_dbus_stop(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_test_dbus_up")]
    pub fn up(&self) {
        unsafe {
            ffi::g_test_dbus_up(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_test_dbus_unset")]
    pub fn unset() {
        unsafe {
            ffi::g_test_dbus_unset();
        }
    }
}
