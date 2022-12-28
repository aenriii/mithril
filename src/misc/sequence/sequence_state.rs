use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;

/// SequenceState is essentially dependency injection for carrying state between sequence closure calls.
/// It operates not by handing off a reference, but instead by handing true ownership of the relied-upon
/// variables that are needed to be carried between calls.

pub struct SequenceState {
    pub state: HashMap<String, Box<dyn Any>>
}
impl SequenceState {
    pub fn new(state: HashMap<String, Box<dyn Any>>) -> Self {
        Self {
            state
        }
    }
    pub fn own<'a, T>(&mut self, key: &str) -> Option<T>
        where T: 'a + Any {
        self.state.remove(key).map(|boxed| *boxed.downcast::<T>().unwrap())
    }
    pub fn disown<'a, T>(&mut self, key: &str, value: T)
        where T: 'a + Any {
        self.state.insert(key.to_string(), Box::new(value));
    }
}