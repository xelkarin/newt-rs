use std::os::raw::c_int;
use components::c_component;

#[repr(C)]
#[allow(dead_code)]
pub enum ExitStructEnum {
    HotKey,
    Component,
    FDReady,
    Timer,
    Error
}

#[repr(C)]
pub union ExitStructUnion {
    pub watch: c_int,
    pub key: c_int,
    pub co: c_component
}

#[repr(C)]
pub struct ExitStruct {
    pub reason: ExitStructEnum,
    pub u: ExitStructUnion
}
