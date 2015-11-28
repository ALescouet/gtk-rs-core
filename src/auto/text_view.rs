// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Container;
#[cfg(gtk_3_6)]
use InputHints;
#[cfg(gtk_3_6)]
use InputPurpose;
use Justification;
use Rectangle;
use Scrollable;
use TextBuffer;
use TextChildAnchor;
use TextMark;
use TextWindowType;
use Widget;
use WrapMode;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct TextView(Object<ffi::GtkTextView>): Widget, Container, Buildable, Scrollable;

    match fn {
        get_type => || ffi::gtk_text_view_get_type(),
    }
}

impl TextView {
    pub fn new() -> TextView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_text_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_buffer(buffer: &TextBuffer) -> TextView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_text_view_new_with_buffer(buffer.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn add_child_at_anchor<T: Upcast<Widget>>(&self, child: &T, anchor: &TextChildAnchor) {
        unsafe {
            ffi::gtk_text_view_add_child_at_anchor(self.to_glib_none().0, child.to_glib_none().0, anchor.to_glib_none().0);
        }
    }

    pub fn add_child_in_window<T: Upcast<Widget>>(&self, child: &T, which_window: TextWindowType, xpos: i32, ypos: i32) {
        unsafe {
            ffi::gtk_text_view_add_child_in_window(self.to_glib_none().0, child.to_glib_none().0, which_window, xpos, ypos);
        }
    }

    //pub fn backward_display_line(&self, iter: /*Ignored*/&TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_backward_display_line() }
    //}

    //pub fn backward_display_line_start(&self, iter: /*Ignored*/&TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_backward_display_line_start() }
    //}

    pub fn buffer_to_window_coords(&self, win: TextWindowType, buffer_x: i32, buffer_y: i32) -> (i32, i32) {
        unsafe {
            let mut window_x = mem::uninitialized();
            let mut window_y = mem::uninitialized();
            ffi::gtk_text_view_buffer_to_window_coords(self.to_glib_none().0, win, buffer_x, buffer_y, &mut window_x, &mut window_y);
            (window_x, window_y)
        }
    }

    //pub fn forward_display_line(&self, iter: /*Ignored*/&TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_forward_display_line() }
    //}

    //pub fn forward_display_line_end(&self, iter: /*Ignored*/&TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_forward_display_line_end() }
    //}

    pub fn get_accepts_tab(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_accepts_tab(self.to_glib_none().0))
        }
    }

    pub fn get_border_window_size(&self, type_: TextWindowType) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_border_window_size(self.to_glib_none().0, type_)
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_buffer(self.to_glib_none().0))
        }
    }

    //pub fn get_cursor_locations(&self, iter: /*Ignored*/Option<&TextIter>) -> (Rectangle, Rectangle) {
    //    unsafe { TODO: call ffi::gtk_text_view_get_cursor_locations() }
    //}

    pub fn get_cursor_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_cursor_visible(self.to_glib_none().0))
        }
    }

    //pub fn get_default_attributes(&self) -> /*Ignored*/TextAttributes {
    //    unsafe { TODO: call ffi::gtk_text_view_get_default_attributes() }
    //}

    pub fn get_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_editable(self.to_glib_none().0))
        }
    }

    pub fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_hadjustment(self.to_glib_none().0))
        }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_indent(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_input_hints(&self) -> InputHints {
        unsafe {
            ffi::gtk_text_view_get_input_hints(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            ffi::gtk_text_view_get_input_purpose(self.to_glib_none().0)
        }
    }

    //pub fn get_iter_at_location(&self, iter: /*Ignored*/TextIter, x: i32, y: i32) {
    //    unsafe { TODO: call ffi::gtk_text_view_get_iter_at_location() }
    //}

    //pub fn get_iter_at_position(&self, iter: /*Ignored*/TextIter, x: i32, y: i32) -> i32 {
    //    unsafe { TODO: call ffi::gtk_text_view_get_iter_at_position() }
    //}

    //pub fn get_iter_location(&self, iter: /*Ignored*/&TextIter) -> Rectangle {
    //    unsafe { TODO: call ffi::gtk_text_view_get_iter_location() }
    //}

    pub fn get_justification(&self) -> Justification {
        unsafe {
            ffi::gtk_text_view_get_justification(self.to_glib_none().0)
        }
    }

    pub fn get_left_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_left_margin(self.to_glib_none().0)
        }
    }

    //pub fn get_line_at_y(&self, target_iter: /*Ignored*/TextIter, y: i32) -> i32 {
    //    unsafe { TODO: call ffi::gtk_text_view_get_line_at_y() }
    //}

    //pub fn get_line_yrange(&self, iter: /*Ignored*/&TextIter) -> (i32, i32) {
    //    unsafe { TODO: call ffi::gtk_text_view_get_line_yrange() }
    //}

    #[cfg(gtk_3_16)]
    pub fn get_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_monospace(self.to_glib_none().0))
        }
    }

    pub fn get_overwrite(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_overwrite(self.to_glib_none().0))
        }
    }

    pub fn get_pixels_above_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_above_lines(self.to_glib_none().0)
        }
    }

    pub fn get_pixels_below_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_below_lines(self.to_glib_none().0)
        }
    }

    pub fn get_pixels_inside_wrap(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_inside_wrap(self.to_glib_none().0)
        }
    }

    pub fn get_right_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_right_margin(self.to_glib_none().0)
        }
    }

    //pub fn get_tabs(&self) -> /*Ignored*/pango::TabArray {
    //    unsafe { TODO: call ffi::gtk_text_view_get_tabs() }
    //}

    pub fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_vadjustment(self.to_glib_none().0))
        }
    }

    pub fn get_visible_rect(&self) -> Rectangle {
        unsafe {
            let mut visible_rect = Rectangle::uninitialized();
            ffi::gtk_text_view_get_visible_rect(self.to_glib_none().0, visible_rect.to_glib_none_mut().0);
            visible_rect
        }
    }

    pub fn get_window(&self, win: TextWindowType) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_window(self.to_glib_none().0, win))
        }
    }

    pub fn get_window_type(&self, window: &gdk::Window) -> TextWindowType {
        unsafe {
            ffi::gtk_text_view_get_window_type(self.to_glib_none().0, window.to_glib_none().0)
        }
    }

    pub fn get_wrap_mode(&self) -> WrapMode {
        unsafe {
            ffi::gtk_text_view_get_wrap_mode(self.to_glib_none().0)
        }
    }

    //pub fn im_context_filter_keypress(&self, event: /*Ignored*/&gdk::EventKey) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_im_context_filter_keypress() }
    //}

    pub fn move_child<T: Upcast<Widget>>(&self, child: &T, xpos: i32, ypos: i32) {
        unsafe {
            ffi::gtk_text_view_move_child(self.to_glib_none().0, child.to_glib_none().0, xpos, ypos);
        }
    }

    pub fn move_mark_onscreen(&self, mark: &TextMark) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_move_mark_onscreen(self.to_glib_none().0, mark.to_glib_none().0))
        }
    }

    //pub fn move_visually(&self, iter: /*Ignored*/&TextIter, count: i32) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_move_visually() }
    //}

    pub fn place_cursor_onscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_place_cursor_onscreen(self.to_glib_none().0))
        }
    }

    pub fn reset_im_context(&self) {
        unsafe {
            ffi::gtk_text_view_reset_im_context(self.to_glib_none().0);
        }
    }

    pub fn scroll_mark_onscreen(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_view_scroll_mark_onscreen(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    //pub fn scroll_to_iter(&self, iter: /*Ignored*/&TextIter, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_scroll_to_iter() }
    //}

    pub fn scroll_to_mark(&self, mark: &TextMark, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) {
        unsafe {
            ffi::gtk_text_view_scroll_to_mark(self.to_glib_none().0, mark.to_glib_none().0, within_margin, use_align.to_glib(), xalign, yalign);
        }
    }

    pub fn set_accepts_tab(&self, accepts_tab: bool) {
        unsafe {
            ffi::gtk_text_view_set_accepts_tab(self.to_glib_none().0, accepts_tab.to_glib());
        }
    }

    pub fn set_border_window_size(&self, type_: TextWindowType, size: i32) {
        unsafe {
            ffi::gtk_text_view_set_border_window_size(self.to_glib_none().0, type_, size);
        }
    }

    pub fn set_buffer(&self, buffer: Option<&TextBuffer>) {
        unsafe {
            ffi::gtk_text_view_set_buffer(self.to_glib_none().0, buffer.to_glib_none().0);
        }
    }

    pub fn set_cursor_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_view_set_cursor_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_editable(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_view_set_editable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::gtk_text_view_set_indent(self.to_glib_none().0, indent);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::gtk_text_view_set_input_hints(self.to_glib_none().0, hints);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::gtk_text_view_set_input_purpose(self.to_glib_none().0, purpose);
        }
    }

    pub fn set_justification(&self, justification: Justification) {
        unsafe {
            ffi::gtk_text_view_set_justification(self.to_glib_none().0, justification);
        }
    }

    pub fn set_left_margin(&self, left_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_left_margin(self.to_glib_none().0, left_margin);
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_monospace(&self, monospace: bool) {
        unsafe {
            ffi::gtk_text_view_set_monospace(self.to_glib_none().0, monospace.to_glib());
        }
    }

    pub fn set_overwrite(&self, overwrite: bool) {
        unsafe {
            ffi::gtk_text_view_set_overwrite(self.to_glib_none().0, overwrite.to_glib());
        }
    }

    pub fn set_pixels_above_lines(&self, pixels_above_lines: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_above_lines(self.to_glib_none().0, pixels_above_lines);
        }
    }

    pub fn set_pixels_below_lines(&self, pixels_below_lines: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_below_lines(self.to_glib_none().0, pixels_below_lines);
        }
    }

    pub fn set_pixels_inside_wrap(&self, pixels_inside_wrap: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_inside_wrap(self.to_glib_none().0, pixels_inside_wrap);
        }
    }

    pub fn set_right_margin(&self, right_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_right_margin(self.to_glib_none().0, right_margin);
        }
    }

    //pub fn set_tabs(&self, tabs: /*Ignored*/&pango::TabArray) {
    //    unsafe { TODO: call ffi::gtk_text_view_set_tabs() }
    //}

    pub fn set_wrap_mode(&self, wrap_mode: WrapMode) {
        unsafe {
            ffi::gtk_text_view_set_wrap_mode(self.to_glib_none().0, wrap_mode);
        }
    }

    //pub fn starts_display_line(&self, iter: /*Ignored*/&TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_view_starts_display_line() }
    //}

    pub fn window_to_buffer_coords(&self, win: TextWindowType, window_x: i32, window_y: i32) -> (i32, i32) {
        unsafe {
            let mut buffer_x = mem::uninitialized();
            let mut buffer_y = mem::uninitialized();
            ffi::gtk_text_view_window_to_buffer_coords(self.to_glib_none().0, win, window_x, window_y, &mut buffer_x, &mut buffer_y);
            (buffer_x, buffer_y)
        }
    }

}
