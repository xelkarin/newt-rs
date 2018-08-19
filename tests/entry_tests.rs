extern crate newt;
use newt::constants::FlagsSense;
use newt::components::Component;
use newt::components::Entry;
use newt::components::Form;
use std::ptr;

use newt::constants::COLORSET_ENTRY;
use newt::constants::COLORSET_DISENTRY;

#[test]
fn entry_create() {
    let entry = Entry::new(0, 0, None, 10, 0);
    assert!(entry.co() != ptr::null());
}

#[test]
fn entry_partial_eq_true() {
    let entry = Entry::new(0, 0, None, 10, 0);
    assert!(entry == entry);
}

#[test]
fn entry_partial_eq_false() {
    let entry = Entry::new(0, 0, None, 10, 0);
    let form = Form::new(0);
    assert!(entry != form);
}

#[test]
fn entry_set_text() {
    let mut entry = Entry::new(0, 0, None, 10, 0);
    let text = entry.get_value();
    assert!(text == "");

    entry.set_text("Hello world!", false);
    let text = entry.get_value();
    assert!(text == "Hello world!");
}

#[test]
fn entry_set_flags() {
    let mut entry = Entry::new(0, 0, None, 10, 0);
    entry.set_flags(0, FlagsSense::Reset);
}

#[test]
fn entry_set_colors() {
    let mut entry = Entry::new(0, 0, None, 10, 0);
    entry.set_colors(COLORSET_ENTRY, COLORSET_DISENTRY);
}