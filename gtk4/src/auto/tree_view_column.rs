// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::CellArea;
use crate::CellLayout;
use crate::CellRenderer;
use crate::SortType;
use crate::TreeIter;
use crate::TreeModel;
use crate::TreeViewColumnSizing;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkTreeViewColumn")]
    pub struct TreeViewColumn(Object<ffi::GtkTreeViewColumn>) @implements Buildable, CellLayout;

    match fn {
        type_ => || ffi::gtk_tree_view_column_get_type(),
    }
}

impl TreeViewColumn {
    #[doc(alias = "gtk_tree_view_column_new")]
    pub fn new() -> TreeViewColumn {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_tree_view_column_new()) }
    }

    #[doc(alias = "gtk_tree_view_column_new_with_area")]
    #[doc(alias = "new_with_area")]
    pub fn with_area(area: &impl IsA<CellArea>) -> TreeViewColumn {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_new_with_area(
                area.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TreeViewColumn`] objects.
    ///
    /// This method returns an instance of [`TreeViewColumnBuilder`](crate::builders::TreeViewColumnBuilder) which can be used to create [`TreeViewColumn`] objects.
    pub fn builder() -> TreeViewColumnBuilder {
        TreeViewColumnBuilder::default()
    }

    #[doc(alias = "gtk_tree_view_column_add_attribute")]
    pub fn add_attribute(
        &self,
        cell_renderer: &impl IsA<CellRenderer>,
        attribute: &str,
        column: i32,
    ) {
        unsafe {
            ffi::gtk_tree_view_column_add_attribute(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_cell_get_position")]
    pub fn cell_get_position(&self, cell_renderer: &impl IsA<CellRenderer>) -> Option<(i32, i32)> {
        unsafe {
            let mut x_offset = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_tree_view_column_cell_get_position(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                x_offset.as_mut_ptr(),
                width.as_mut_ptr(),
            ));
            if ret {
                Some((x_offset.assume_init(), width.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_tree_view_column_cell_get_size")]
    pub fn cell_get_size(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x_offset = mem::MaybeUninit::uninit();
            let mut y_offset = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_tree_view_column_cell_get_size(
                self.to_glib_none().0,
                x_offset.as_mut_ptr(),
                y_offset.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (
                x_offset.assume_init(),
                y_offset.assume_init(),
                width.assume_init(),
                height.assume_init(),
            )
        }
    }

    #[doc(alias = "gtk_tree_view_column_cell_is_visible")]
    pub fn cell_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_cell_is_visible(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_cell_set_cell_data")]
    pub fn cell_set_cell_data(
        &self,
        tree_model: &impl IsA<TreeModel>,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    ) {
        unsafe {
            ffi::gtk_tree_view_column_cell_set_cell_data(
                self.to_glib_none().0,
                tree_model.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                is_expander.into_glib(),
                is_expanded.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tree_view_column_clear_attributes")]
    pub fn clear_attributes(&self, cell_renderer: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_tree_view_column_clear_attributes(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_clicked")]
    pub fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tree_view_column_focus_cell")]
    pub fn focus_cell(&self, cell: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_tree_view_column_focus_cell(
                self.to_glib_none().0,
                cell.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_alignment")]
    #[doc(alias = "get_alignment")]
    pub fn alignment(&self) -> f32 {
        unsafe { ffi::gtk_tree_view_column_get_alignment(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_button")]
    #[doc(alias = "get_button")]
    pub fn button(&self) -> Widget {
        unsafe { from_glib_none(ffi::gtk_tree_view_column_get_button(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_clickable")]
    #[doc(alias = "get_clickable")]
    pub fn is_clickable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_clickable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_expand")]
    #[doc(alias = "get_expand")]
    pub fn expands(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_column_get_expand(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_fixed_width")]
    #[doc(alias = "get_fixed_width")]
    pub fn fixed_width(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_fixed_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_max_width")]
    #[doc(alias = "get_max_width")]
    pub fn max_width(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_max_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_min_width")]
    #[doc(alias = "get_min_width")]
    pub fn min_width(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_min_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_reorderable")]
    #[doc(alias = "get_reorderable")]
    pub fn is_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_reorderable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_resizable")]
    #[doc(alias = "get_resizable")]
    pub fn is_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_resizable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_sizing")]
    #[doc(alias = "get_sizing")]
    pub fn sizing(&self) -> TreeViewColumnSizing {
        unsafe { from_glib(ffi::gtk_tree_view_column_get_sizing(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_sort_column_id")]
    #[doc(alias = "get_sort_column_id")]
    pub fn sort_column_id(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_sort_column_id(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_sort_indicator")]
    #[doc(alias = "get_sort_indicator")]
    pub fn is_sort_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_indicator(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_sort_order")]
    #[doc(alias = "get_sort_order")]
    pub fn sort_order(&self) -> SortType {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_order(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_tree_view_column_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_tree_view")]
    #[doc(alias = "get_tree_view")]
    pub fn tree_view(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_tree_view(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_view_column_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_column_get_visible(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_widget")]
    #[doc(alias = "get_widget")]
    pub fn widget(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_tree_view_column_get_widget(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_view_column_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_get_x_offset")]
    #[doc(alias = "get_x_offset")]
    pub fn x_offset(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_column_get_x_offset(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_view_column_pack_end")]
    pub fn pack_end(&self, cell: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_end(
                self.to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_pack_start")]
    pub fn pack_start(&self, cell: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_start(
                self.to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_queue_resize")]
    pub fn queue_resize(&self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_alignment")]
    pub fn set_alignment(&self, xalign: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.to_glib_none().0, xalign);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_cell_data_func")]
    pub fn set_cell_data_func<
        P: Fn(&TreeViewColumn, &CellRenderer, &TreeModel, &TreeIter) + 'static,
    >(
        &self,
        cell_renderer: &impl IsA<CellRenderer>,
        func: P,
    ) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<
            P: Fn(&TreeViewColumn, &CellRenderer, &TreeModel, &TreeIter) + 'static,
        >(
            tree_column: *mut ffi::GtkTreeViewColumn,
            cell: *mut ffi::GtkCellRenderer,
            tree_model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) {
            let tree_column = from_glib_borrow(tree_column);
            let cell = from_glib_borrow(cell);
            let tree_model = from_glib_borrow(tree_model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            (*callback)(&tree_column, &cell, &tree_model, &iter);
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&TreeViewColumn, &CellRenderer, &TreeModel, &TreeIter) + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_tree_view_column_set_cell_data_func(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_clickable")]
    pub fn set_clickable(&self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.to_glib_none().0, clickable.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_expand")]
    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.to_glib_none().0, expand.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_fixed_width")]
    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_fixed_width(self.to_glib_none().0, fixed_width);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_max_width")]
    pub fn set_max_width(&self, max_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_max_width(self.to_glib_none().0, max_width);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_min_width")]
    pub fn set_min_width(&self, min_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_min_width(self.to_glib_none().0, min_width);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_reorderable")]
    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(
                self.to_glib_none().0,
                reorderable.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_resizable")]
    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.to_glib_none().0, resizable.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_sizing")]
    pub fn set_sizing(&self, type_: TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_sort_column_id")]
    pub fn set_sort_column_id(&self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.to_glib_none().0, sort_column_id);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_sort_indicator")]
    pub fn set_sort_indicator(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(
                self.to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_sort_order")]
    pub fn set_sort_order(&self, order: SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.to_glib_none().0, order.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_spacing")]
    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_tree_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_visible")]
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_widget")]
    pub fn set_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tree_view_column_set_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "cell-area")]
    pub fn cell_area(&self) -> Option<CellArea> {
        glib::ObjectExt::property(self, "cell-area")
    }

    #[doc(alias = "clicked")]
    pub fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clicked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clicked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "alignment")]
    pub fn connect_alignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alignment_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::alignment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alignment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "clickable")]
    pub fn connect_clickable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clickable_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::clickable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clickable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expand")]
    pub fn connect_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expand_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::expand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fixed-width")]
    pub fn connect_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fixed_width_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::fixed-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fixed_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-width")]
    pub fn connect_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::max-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-width")]
    pub fn connect_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_width_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::min-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reorderable")]
    pub fn connect_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reorderable_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::reorderable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reorderable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resizable")]
    pub fn connect_resizable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resizable_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::resizable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resizable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sizing")]
    pub fn connect_sizing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sizing_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::sizing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sizing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sort-column-id")]
    pub fn connect_sort_column_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_column_id_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::sort-column-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_column_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sort-indicator")]
    pub fn connect_sort_indicator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_indicator_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::sort-indicator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_indicator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sort-order")]
    pub fn connect_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_order_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::sort-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_order_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    pub fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "widget")]
    pub fn connect_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_widget_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width")]
    pub fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "x-offset")]
    pub fn connect_x_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_x_offset_trampoline<F: Fn(&TreeViewColumn) + 'static>(
            this: *mut ffi::GtkTreeViewColumn,
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
                b"notify::x-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_x_offset_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TreeViewColumn {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TreeViewColumn`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TreeViewColumnBuilder {
    alignment: Option<f32>,
    cell_area: Option<CellArea>,
    clickable: Option<bool>,
    expand: Option<bool>,
    fixed_width: Option<i32>,
    max_width: Option<i32>,
    min_width: Option<i32>,
    reorderable: Option<bool>,
    resizable: Option<bool>,
    sizing: Option<TreeViewColumnSizing>,
    sort_column_id: Option<i32>,
    sort_indicator: Option<bool>,
    sort_order: Option<SortType>,
    spacing: Option<i32>,
    title: Option<String>,
    visible: Option<bool>,
    widget: Option<Widget>,
}

impl TreeViewColumnBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TreeViewColumnBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TreeViewColumn`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TreeViewColumn {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref alignment) = self.alignment {
            properties.push(("alignment", alignment));
        }
        if let Some(ref cell_area) = self.cell_area {
            properties.push(("cell-area", cell_area));
        }
        if let Some(ref clickable) = self.clickable {
            properties.push(("clickable", clickable));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref fixed_width) = self.fixed_width {
            properties.push(("fixed-width", fixed_width));
        }
        if let Some(ref max_width) = self.max_width {
            properties.push(("max-width", max_width));
        }
        if let Some(ref min_width) = self.min_width {
            properties.push(("min-width", min_width));
        }
        if let Some(ref reorderable) = self.reorderable {
            properties.push(("reorderable", reorderable));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref sizing) = self.sizing {
            properties.push(("sizing", sizing));
        }
        if let Some(ref sort_column_id) = self.sort_column_id {
            properties.push(("sort-column-id", sort_column_id));
        }
        if let Some(ref sort_indicator) = self.sort_indicator {
            properties.push(("sort-indicator", sort_indicator));
        }
        if let Some(ref sort_order) = self.sort_order {
            properties.push(("sort-order", sort_order));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new::<TreeViewColumn>(&properties)
    }

    pub fn alignment(mut self, alignment: f32) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn cell_area(mut self, cell_area: &impl IsA<CellArea>) -> Self {
        self.cell_area = Some(cell_area.clone().upcast());
        self
    }

    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = Some(clickable);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn fixed_width(mut self, fixed_width: i32) -> Self {
        self.fixed_width = Some(fixed_width);
        self
    }

    pub fn max_width(mut self, max_width: i32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn min_width(mut self, min_width: i32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = Some(reorderable);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn sizing(mut self, sizing: TreeViewColumnSizing) -> Self {
        self.sizing = Some(sizing);
        self
    }

    pub fn sort_column_id(mut self, sort_column_id: i32) -> Self {
        self.sort_column_id = Some(sort_column_id);
        self
    }

    pub fn sort_indicator(mut self, sort_indicator: bool) -> Self {
        self.sort_indicator = Some(sort_indicator);
        self
    }

    pub fn sort_order(mut self, sort_order: SortType) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn widget(mut self, widget: &impl IsA<Widget>) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for TreeViewColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeViewColumn")
    }
}
