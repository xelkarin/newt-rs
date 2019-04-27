use crate::intern::funcs::*;

pub struct SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    func: FN,
    data: Option<T>
}

impl<FN, T> SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    ///
    /// Create a new `SuspendCallback` to be called when a suspend (Ctrl-Z)
    /// event occurs.
    ///
    /// * `function` - function or closure to be called when a suspend
    ///                event occurs
    /// * `data`     - optional data to pass to the function
    ///
    pub fn new(function: FN, data: Option<T>)
      -> Box<SuspendCallback<FN, T>> {
        let cb = Box::new(SuspendCallback { func: function, data: data });
        newt_set_suspend_callback(cb.as_ref());
        return cb;
    }

    pub(crate) fn call(&mut self) {
        (self.func)(self.data.as_ref())
    }
}

impl<FN, T> Drop for SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    fn drop(&mut self) {
        newt_unset_suspend_callback();
    }
}