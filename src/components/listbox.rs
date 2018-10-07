extern crate std;
use std::cmp::PartialEq;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Drop;
use std::os::raw::{c_char, c_void};
use ptr;

use components::c_component;
use components::Component;
use components::data::Data;
use components::form::ExitReason;
use constants::FlagsSense;
use intern::ffi::newt::listbox::*;
use intern::ffi::newt::component::newtComponentDestroy;

pub struct Listbox<D: Data> {
    co: c_component,
    attached_to_form: bool,
    data: PhantomData<D>
}

impl<D: Data> Listbox<D> {
    pub fn new(left: i32, top: i32, height: i32, flags: i32) -> Listbox<D> {
        Listbox {
            co: unsafe { newtListbox(left, top, height, flags) },
            attached_to_form: false,
            data: PhantomData
        }
    }

    pub fn set_width(&self, width: i32) {
        unsafe { newtListboxSetWidth(self.co, width); }
    }

    pub fn item_count(&self) -> i32 {
        unsafe { newtListboxItemCount(self.co) }
    }

    pub fn append_entry(&self, text: &str, data: D) -> i32 {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtListboxAppendEntry(self.co, c_str.as_ptr(), data.newt_to_ptr())
        }
    }

    pub fn insert_entry(&self, text: &str, data: D, key: D) -> i32 {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtListboxInsertEntry(self.co, c_str.as_ptr(), data.newt_to_ptr(),
                                   key.newt_to_ptr())
        }
    }

    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtListboxGetCurrent(self.co) };
        if c_data == ptr::null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    pub fn set_current(&self, num: i32) {
        unsafe { newtListboxSetCurrent(self.co, num); }
    }

    pub fn set_current_by_key(&self, key: D) {
        unsafe { newtListboxSetCurrentByKey(self.co, key.newt_to_ptr()); }
    }

    pub fn get_entry(&self, num: i32) -> (&str, D) {
        let c_str: *mut c_char = ptr::null_mut();
        let c_data: *mut c_void = ptr::null_mut();
        unsafe { newtListboxGetEntry(self.co, num, &c_str, &c_data); }
        let c_str = unsafe { CStr::from_ptr(c_str) };
        (c_str.to_str().unwrap(), D::newt_from_ptr(c_data))
    }

    pub fn set_entry(&self, num: i32, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe { newtListboxSetEntry(self.co, num, c_str.as_ptr()); }
    }

    pub fn set_data(&self, num: i32, data: D) {
        unsafe { newtListboxSetData(self.co, num, data.newt_to_ptr()); }
    }

    pub fn delete_entry(&self, data: D) -> i32 {
        unsafe { newtListboxDeleteEntry(self.co, data.newt_to_ptr()) }
    }

    pub fn clear(&self) {
        unsafe { newtListboxClear(self.co); }
    }

    pub fn get_selection(&self) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe { newtListboxGetSelection(self.co, &mut numitems) };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn select_item(&self, key: D, sense: FlagsSense) {
        unsafe { newtListboxSelectItem(self.co, key.newt_to_ptr(), sense) };
    }

    pub fn clear_selection(&self) {
        unsafe { newtListboxClearSelection(self.co) };
    }
}

impl<D: Data> Component for Listbox<D> {
    fn co(&self) -> c_component {
        self.co
    }

    fn attach_to_form(&mut self) {
        self.attached_to_form = true;
    }

    fn attached_to_form(&self) -> bool {
        self.attached_to_form
    }
}

impl<D: Data> Drop for Listbox<D> {
    fn drop(&mut self) {
        if !self.attached_to_form() {
            unsafe { newtComponentDestroy(self.co()); }
        }
    }
}

impl<D: Data, Rhs: Component> PartialEq<Rhs> for Listbox<D> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co == other.co()
    }
}

impl<D: Data> PartialEq<Box<dyn (Component)>> for Listbox<D> {
    fn eq(&self, other: &Box<dyn (Component)>) -> bool {
        self.co == other.co()
    }
}

impl<D: Data> PartialEq<ExitReason> for Listbox<D> {
    fn eq(&self, other: &ExitReason) -> bool {
        other == self
    }
}
