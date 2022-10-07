// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkDropTarget")]
    pub struct DropTarget(Object<ffi::GtkDropTarget, ffi::GtkDropTargetClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_drop_target_get_type(),
    }
}

impl DropTarget {
    #[doc(alias = "gtk_drop_target_new")]
    pub fn new(type_: glib::types::Type, actions: gdk::DragAction) -> DropTarget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_drop_target_new(
                type_.into_glib(),
                actions.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DropTarget`] objects.
    ///
    /// This method returns an instance of [`DropTargetBuilder`](crate::builders::DropTargetBuilder) which can be used to create [`DropTarget`] objects.
    pub fn builder() -> DropTargetBuilder {
        DropTargetBuilder::default()
    }

    #[doc(alias = "gtk_drop_target_get_actions")]
    #[doc(alias = "get_actions")]
    pub fn actions(&self) -> gdk::DragAction {
        unsafe { from_glib(ffi::gtk_drop_target_get_actions(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_drop_target_get_current_drop")]
    #[doc(alias = "get_current_drop")]
    pub fn current_drop(&self) -> Option<gdk::Drop> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_current_drop(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[doc(alias = "gtk_drop_target_get_drop")]
    #[doc(alias = "get_drop")]
    pub fn drop(&self) -> Option<gdk::Drop> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_drop(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_target_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> Option<gdk::ContentFormats> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_target_get_preload")]
    #[doc(alias = "get_preload")]
    pub fn is_preload(&self) -> bool {
        unsafe { from_glib(ffi::gtk_drop_target_get_preload(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_target_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> Option<glib::Value> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_value(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_target_reject")]
    pub fn reject(&self) {
        unsafe {
            ffi::gtk_drop_target_reject(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_drop_target_set_actions")]
    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_drop_target_set_actions(self.to_glib_none().0, actions.into_glib());
        }
    }

    #[doc(alias = "gtk_drop_target_set_preload")]
    pub fn set_preload(&self, preload: bool) {
        unsafe {
            ffi::gtk_drop_target_set_preload(self.to_glib_none().0, preload.into_glib());
        }
    }

    #[doc(alias = "accept")]
    pub fn connect_accept<F: Fn(&Self, &gdk::Drop) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_trampoline<F: Fn(&DropTarget, &gdk::Drop) -> bool + 'static>(
            this: *mut ffi::GtkDropTarget,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "enter")]
    pub fn connect_enter<F: Fn(&Self, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "leave")]
    pub fn connect_leave<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "motion")]
    pub fn connect_motion<F: Fn(&Self, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "actions")]
    pub fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "current-drop")]
    pub fn connect_current_drop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_drop_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[doc(alias = "drop")]
    pub fn connect_drop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preload")]
    pub fn connect_preload_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_preload_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::preload\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_preload_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    pub fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DropTarget {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DropTarget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DropTargetBuilder {
    actions: Option<gdk::DragAction>,
    formats: Option<gdk::ContentFormats>,
    preload: Option<bool>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl DropTargetBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DropTargetBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DropTarget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DropTarget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref actions) = self.actions {
            properties.push(("actions", actions));
        }
        if let Some(ref formats) = self.formats {
            properties.push(("formats", formats));
        }
        if let Some(ref preload) = self.preload {
            properties.push(("preload", preload));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<DropTarget>(&properties)
    }

    pub fn actions(mut self, actions: gdk::DragAction) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn formats(mut self, formats: &gdk::ContentFormats) -> Self {
        self.formats = Some(formats.clone());
        self
    }

    pub fn preload(mut self, preload: bool) -> Self {
        self.preload = Some(preload);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for DropTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DropTarget")
    }
}
