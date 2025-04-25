use std::rc::Rc;
use std::cell::RefCell;
use gloo_timers::callback::Interval;

#[derive(Clone, Debug)]

pub struct Throttler<T>
where
    T: Clone + 'static,
{
    last_value: Rc<RefCell<Option<T>>>,
}

impl<T> Throttler<T>
where
    T: Clone + 'static,
{
    pub fn new<F>(mut handler: F, millis: u32) -> Self
    where
        F: FnMut(T) + 'static,
    {
        let last_value = Rc::new(RefCell::new(None::<T>));
        let last_clone = last_value.clone();

        // Interval loop
        Interval::new(millis, move || {
            if let Some(val) = last_clone.borrow_mut().take() {
                handler(val);
            }
        })
            .forget();

        Self { last_value }
    }

    pub fn send(&self, value: T) {
        *self.last_value.borrow_mut() = Some(value);
    }
}
