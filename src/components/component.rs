extern crate std;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::os::raw::c_int;

use components::c_component;
use components::form::ExitReason;

pub trait Component {
    fn co(&self) -> c_component;
    fn attach_to_form(&mut self);
    fn attached_to_form(&self) -> bool;

    fn takes_focus(&mut self, value: bool) {
        #[link(name="newt")]
        extern "C" {
            fn newtComponentTakesFocus(co: c_component, val: c_int);
        }

        unsafe { newtComponentTakesFocus(self.co(), value as c_int); }
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Component {{ {:p} }}", self.co())
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Component) -> bool {
        self.co() == other.co()
    }
}

impl PartialEq<ExitReason> for Component {
    fn eq(&self, other: &ExitReason) -> bool {
        if let &ExitReason::Component(ref component) = other {
            return self.co() == component.co()
        }
        return false;
    }
}

impl<Rhs: Component> PartialEq<Rhs> for Box<Component> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co() == other.co()
    }
}