use std::error::Error;
use crate::stream::traits::SeekableInputStream;

pub trait Source {
    fn open(&self) -> Result<Box<dyn SeekableInputStream>, Box<dyn Error>>;
    fn reset(&mut self);
}