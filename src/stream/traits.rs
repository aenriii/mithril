use std::error::Error;

/// A stream that can be read from.
pub trait InputStream {
    /// Read one byte from the stream.
    fn read_one(&mut self) -> Result<u8, Box<dyn Error>>;
    /// Read some bytes from the stream.
    fn read_some(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>;
    /// Read some bytes from the stream at the given offset.
    fn read_some_at(&mut self, buf: &mut [u8], offset: usize) -> Result<usize, Box<dyn Error>>;
    /// Skip some bytes from the stream.
    fn skip(&mut self, n: usize) -> Result<(), Box<dyn Error>>;
}
/// OutputStream is a trait for writing bytes to a stream.
pub trait OutputStream {
    /// Write one byte to the stream.
    fn write_one(&mut self, byte: u8) -> Result<(), Box<dyn Error>>;
    /// Write some bytes to the stream.
    fn write_some(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>>;
    /// Write some bytes to the stream at the given offset.
    fn write_some_at(&mut self, buf: &[u8], pos: usize) -> Result<(), Box<dyn Error>>;
}
/// Given trait for any stream that is seekable.
pub trait Seekable {
    /// Seek to the given position.
    fn seek(&mut self, pos: usize) -> Result<(), Box<dyn Error>>;
    /// Get the current position.
    fn pos(&self) -> usize;
    /// Get the length of the stream.
    fn length(&self) -> usize;
}
pub trait SeekableInputStream: InputStream + Seekable {}
pub trait SeekableOutputStream: OutputStream + Seekable {}