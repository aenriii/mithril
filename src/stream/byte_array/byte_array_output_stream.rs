use std::error::Error;
use crate::stream::traits::{OutputStream, Seekable, SeekableOutputStream};

pub struct ByteArrayOutputStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl OutputStream for ByteArrayOutputStream {
    fn write_one(&mut self, byte: u8) -> Result<(), Box<dyn Error>> {
        if self.pos >= self.bytes.len() {
            self.bytes.push(byte);
        } else {
            self.bytes[self.pos] = byte;
        }
        self.pos += 1;
        Ok(())
    }

    fn write_some(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.pos + buf.len() > self.bytes.len() {
            self.bytes.extend_from_slice(buf);
        } else {
            self.bytes[self.pos..self.pos + buf.len()].copy_from_slice(buf);
        }
        self.pos += buf.len();
        Ok(())
    }

    fn write_some_at(&mut self, buf: &[u8], pos: usize) -> Result<(), Box<dyn Error>> {
        if pos + buf.len() > self.bytes.len() {
            self.bytes.extend_from_slice(buf);
        } else {
            self.bytes[pos..pos + buf.len()].copy_from_slice(buf);
        }
        Ok(())
    }
}

impl Seekable for ByteArrayOutputStream {
    fn seek(&mut self, pos: usize) -> Result<(), Box<dyn Error>> {
        self.pos = pos;
        Ok(())
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl SeekableOutputStream for ByteArrayOutputStream {}
