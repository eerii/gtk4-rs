// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use cairo;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DrawingArea(Object<gtk_sys::GtkDrawingArea, gtk_sys::GtkDrawingAreaClass, DrawingAreaClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_drawing_area_new()).unsafe_cast()
        }
    }
}

impl Default for DrawingArea {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DRAWING_AREA: Option<&DrawingArea> = None;

pub trait DrawingAreaExt: 'static {
    fn get_content_height(&self) -> i32;

    fn get_content_width(&self) -> i32;

    fn set_content_height(&self, height: i32);

    fn set_content_width(&self, width: i32);

    fn set_draw_func(&self, draw_func: Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>);

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DrawingArea>> DrawingAreaExt for O {
    fn get_content_height(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_drawing_area_get_content_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_content_width(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_drawing_area_get_content_width(self.as_ref().to_glib_none().0)
        }
    }

    fn set_content_height(&self, height: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_content_width(&self, width: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_draw_func(&self, draw_func: Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>) {
        let draw_func_data: Box_<Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>> = Box::new(draw_func);
        unsafe extern "C" fn draw_func_func(drawing_area: *mut gtk_sys::GtkDrawingArea, cr: *mut cairo_sys::cairo_t, width: libc::c_int, height: libc::c_int, user_data: glib_sys::gpointer) {
            let drawing_area = from_glib_borrow(drawing_area);
            let cr = from_glib_borrow(cr);
            let callback: &Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>> = &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&drawing_area, &cr, width, height)
            } else {
                panic!("cannot get closure...")
            };
        }
        let draw_func = if draw_func_data.is_some() { Some(draw_func_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>> = draw_func_data;
        unsafe {
            gtk_sys::gtk_drawing_area_set_draw_func(self.as_ref().to_glib_none().0, draw_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::content-height\0".as_ptr() as *const _,
                Some(transmute(notify_content_height_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::content-width\0".as_ptr() as *const _,
                Some(transmute(notify_content_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_content_height_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkDrawingArea, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DrawingArea> {
    let f: &F = &*(f as *const F);
    f(&DrawingArea::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_content_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkDrawingArea, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DrawingArea> {
    let f: &F = &*(f as *const F);
    f(&DrawingArea::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DrawingArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawingArea")
    }
}
