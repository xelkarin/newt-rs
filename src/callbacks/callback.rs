use crate::components::Component;
use crate::intern::funcs::*;
use newt_sys::*;

pub struct Callback<'a, FN: 'a, T: 'a>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    func: FN,
    components: Vec<(&'a Component, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> Callback<'a, FN, T>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    ///
    /// Create a new `Callback` using the function or closure `function` and
    /// associate it with `component`.
    ///
    /// * `component` - the `Component` to which the callback will be attached
    /// * `function`  - the function or closure to attach to the `Component`
    /// * `data`      - optional data to pass to the function
    ///
    pub fn new(component: &'a Component, function: FN, data: Option<T>)
      -> Box<Callback<'a, FN, T>> {

        let cb = Box::new(Callback {
            func: function,
            components: vec![(component, data)]
        });
        newt_set_callback(component.co(), cb.as_ref());
        return cb;
    }

    pub fn add_component(&mut self, component: &'a Component, data: Option<T>)
    {
        self.components.push((component, data));
        newt_set_callback(component.co(), &self);
    }

    pub(crate) fn call(&mut self, co: newtComponent) {
        for (component, data) in self.components.iter() {
            if component.co() == co {
                (self.func)(Some(*component), data.as_ref());
            }
        }
    }
}

impl<'a, FN: 'a, T: 'a> Drop for Callback<'a, FN, T>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    fn drop(&mut self) {
        for (component, _data) in self.components.iter() {
            newt_unset_callback(*component)
        }
    }
}