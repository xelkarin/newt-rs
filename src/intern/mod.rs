#[macro_use]
pub mod macros;
#[cfg(feature = "asm")]
pub mod asm;
pub mod data;
pub mod funcs;

use libc::c_void;
use newt_sys::*;

pub trait Child {
    fn add_to_parent(&mut self) -> Result<(), &'static str>;
    fn added_to_parent(&self) -> bool;
}

pub trait ComponentPtr {
    fn ptr(&self) -> *mut c_void;
    fn as_co(&self) -> newtComponent;
    fn as_grid(&self) -> newtGrid;
}

pub trait GridElementType {
    fn grid_element_type(&self) -> u32;
}
