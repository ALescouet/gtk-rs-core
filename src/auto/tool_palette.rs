// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Container;
use Orientable;
use Scrollable;
use ToolItem;
use ToolItemGroup;
use ToolPaletteDragTargets;
use ToolbarStyle;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ToolPalette(Object<ffi::GtkToolPalette>): Widget, Container, Buildable, Orientable, Scrollable;

    match fn {
        get_type => || ffi::gtk_tool_palette_get_type(),
    }
}

impl ToolPalette {
    pub fn new() -> ToolPalette {
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_palette_new()).downcast_unchecked()
        }
    }

    //pub fn add_drag_dest<T: Upcast<Widget>>(&self, widget: &T, flags: DestDefaults, targets: ToolPaletteDragTargets, actions: gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_tool_palette_add_drag_dest() }
    //}

    //pub fn get_drag_item(&self, selection: /*Ignored*/&SelectionData) -> Option<Widget> {
    //    unsafe { TODO: call ffi::gtk_tool_palette_get_drag_item() }
    //}

    pub fn get_drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_group(self.to_glib_none().0, x, y))
        }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_item(self.to_glib_none().0, x, y))
        }
    }

    pub fn get_exclusive(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_exclusive(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    pub fn get_expand(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_expand(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    pub fn get_group_position(&self, group: &ToolItemGroup) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_group_position(self.to_glib_none().0, group.to_glib_none().0)
        }
    }

    pub fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_hadjustment(self.to_glib_none().0))
        }
    }

    pub fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_icon_size(self.to_glib_none().0)
        }
    }

    pub fn get_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_palette_get_style(self.to_glib_none().0)
        }
    }

    pub fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_vadjustment(self.to_glib_none().0))
        }
    }

    pub fn set_drag_source(&self, targets: ToolPaletteDragTargets) {
        unsafe {
            ffi::gtk_tool_palette_set_drag_source(self.to_glib_none().0, targets);
        }
    }

    pub fn set_exclusive(&self, group: &ToolItemGroup, exclusive: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_exclusive(self.to_glib_none().0, group.to_glib_none().0, exclusive.to_glib());
        }
    }

    pub fn set_expand(&self, group: &ToolItemGroup, expand: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_expand(self.to_glib_none().0, group.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn set_group_position(&self, group: &ToolItemGroup, position: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_group_position(self.to_glib_none().0, group.to_glib_none().0, position);
        }
    }

    pub fn set_icon_size(&self, icon_size: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_icon_size(self.to_glib_none().0, icon_size);
        }
    }

    pub fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_tool_palette_set_style(self.to_glib_none().0, style);
        }
    }

    pub fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_icon_size(self.to_glib_none().0);
        }
    }

    pub fn unset_style(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_style(self.to_glib_none().0);
        }
    }

    //pub fn get_drag_target_group() -> /*Ignored*/TargetEntry {
    //    unsafe { TODO: call ffi::gtk_tool_palette_get_drag_target_group() }
    //}

    //pub fn get_drag_target_item() -> /*Ignored*/TargetEntry {
    //    unsafe { TODO: call ffi::gtk_tool_palette_get_drag_target_item() }
    //}

}
