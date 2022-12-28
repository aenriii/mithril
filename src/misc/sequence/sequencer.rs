use crate::misc::sequence::sequence_state::SequenceState;

pub struct Sequence<Output> {
     pub state: SequenceState,
     pub closure: Self::SeqClosure
}
impl <Output> Sequence<Output> {
    type Output = Output;
    type SeqClosure = Box<dyn FnMut(&mut SequenceState) -> Self::Output>;
    pub fn new(state: SequenceState, closure: Self::SeqClosure) -> Self {
        Self {
            state,
            closure
        }
    }
}
impl <T> Iterator for Sequence<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.closure)(&mut self.state))
    }
}
