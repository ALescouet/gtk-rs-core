// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
use crate::value::FromValue;
use crate::value::ToValue;
use crate::StaticType;
use crate::Type;
use bitflags::bitflags;
use std::fmt;

bitflags! {
    pub struct BindingFlags: u32 {
        const DEFAULT = 0;
        const BIDIRECTIONAL = 1;
        const SYNC_CREATE = 2;
        const INVERT_BOOLEAN = 4;
    }
}

impl fmt::Display for BindingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for BindingFlags {
    type GlibType = gobject_ffi::GBindingFlags;

    fn into_glib(self) -> gobject_ffi::GBindingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GBindingFlags> for BindingFlags {
    unsafe fn from_glib(value: gobject_ffi::GBindingFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for BindingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_binding_flags_get_type()) }
    }
}

impl crate::value::ValueType for BindingFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BindingFlags {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        from_glib(crate::gobject_ffi::g_value_get_flags(
            value.to_glib_none().0,
        ))
    }
}

impl ToValue for BindingFlags {
    fn to_value(&self) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            crate::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct SignalFlags: u32 {
        const RUN_FIRST = 1;
        const RUN_LAST = 2;
        const RUN_CLEANUP = 4;
        const NO_RECURSE = 8;
        const DETAILED = 16;
        const ACTION = 32;
        const NO_HOOKS = 64;
        const MUST_COLLECT = 128;
        const DEPRECATED = 256;
    }
}

impl fmt::Display for SignalFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for SignalFlags {
    type GlibType = gobject_ffi::GSignalFlags;

    fn into_glib(self) -> gobject_ffi::GSignalFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GSignalFlags> for SignalFlags {
    unsafe fn from_glib(value: gobject_ffi::GSignalFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}
