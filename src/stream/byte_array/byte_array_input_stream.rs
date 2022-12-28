use std::error::Error;
use crate::stream::traits::{InputStream, Seekable, SeekableInputStream};

struct ByteArrayInputStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl InputStream for ByteArrayInputStream {
    fn read_one(&mut self) -> Result<u8, Box<dyn Error>> {
        if self.pos >= self.bytes.len() {
            return Err("EOF".into());
        }
        let byte = self.bytes[self.pos];
        self.pos += 1;
        Ok(byte)
    }

    fn read_some(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        if self.pos >= self.bytes.len() {
            return Err("EOF".into());
        }
        if self.pos + buf.len() > self.bytes.len() {
            buf.copy_from_slice(&self.bytes[self.pos..]);
            self.pos = self.bytes.len();
            return Ok(buf.len());
        }
        let n = std::cmp::min(buf.len(), self.bytes.len() - self.pos);
        buf[..n].copy_from_slice(&self.bytes[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }

    fn read_some_at(&mut self, buf: &mut [u8], offset: usize) -> Result<usize, Box<dyn Error>> {
        // use offset as offset into buf

        if self.pos >= self.bytes.len() {
            return Err("EOF".into());
        }
        if self.pos + offset > self.bytes.len() {
            buf.copy_from_slice(&self.bytes[self.pos..]);
            self.pos = self.bytes.len();
            return Ok(buf.len());
        }
        let n = std::cmp::min(buf.len() - offset, self.bytes.len() - self.pos);
        buf[offset..offset + n].copy_from_slice(&self.bytes[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)

    }

    fn skip(&mut self, n: usize) -> Result<(), Box<dyn Error>> {
        if self.pos + n > self.bytes.len() {
            return Err("EOF".into());
        }
        self.pos += n;
        Ok(())
    }
}

impl Seekable for ByteArrayInputStream {
    fn seek(&mut self, pos: usize) -> Result<(), Box<dyn Error>> {
        if pos > self.bytes.len() {
            return Err("EOF".into());
        }
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

impl SeekableInputStream for ByteArrayInputStream {}

impl ByteArrayInputStream {
    pub fn new(bytes: Vec<u8>) -> Self {
        ByteArrayInputStream {
            bytes,
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_one() {
        let mut stream = ByteArrayInputStream::new(vec![1, 2, 3, 4]);
        assert_eq!(stream.read_one().unwrap(), 1);
        assert_eq!(stream.read_one().unwrap(), 2);
        assert_eq!(stream.read_one().unwrap(), 3);
        assert_eq!(stream.read_one().unwrap(), 4);
        assert!(stream.read_one().is_err());
    }

    #[test]
    fn test_read_some() {
        let mut stream = ByteArrayInputStream::new(vec![1, 2, 3, 4]);
        let mut buf = [0u8; 3];
        assert_eq!(stream.read_some(&mut buf).unwrap(), 3);
        assert_eq!(buf, [1, 2, 3]);
        assert_eq!(stream.read_some(&mut buf).unwrap(), 1);
        assert_eq!(buf, [1, 2, 4]);
        assert!(stream.read_some(&mut buf).is_err());
    }

    #[test]
    fn test_read_some_at() {
        let mut stream = ByteArrayInputStream::new(vec![1, 2, 3, 4]);
        let mut buf = [0u8; 3];
        assert_eq!(stream.read_some_at(&mut buf, 1).unwrap(), 2);
        assert_eq!(buf, [0, 1, 2]);
        assert_eq!(stream.read_some_at(&mut buf, 0).unwrap(), 1);
        assert_eq!(buf, [3, 1, 2]);
        assert!(stream.read_some_at(&mut buf, 0).is_err());
    }

    #[test]
    fn test_skip() {
        let mut stream = ByteArrayInputStream::new(vec![1, 2, 3, 4]);
        assert_eq!(stream.read_one().unwrap(), 1);
        assert_eq!(stream.read_one().unwrap(), 2);
        stream.skip(1).unwrap();
        assert_eq!(stream.read_one().unwrap(), 4);
        assert!(stream.read_one().is_err());
    }

    #[test]
    fn test_seek() {
        let mut stream = ByteArrayInputStream::new(vec![1, 2, 3, 4]);
        stream.seek(2).unwrap();
        assert_eq!(stream.read_one().unwrap(), 3);
        assert_eq!(stream.read_one().unwrap(), 4);
        assert!(stream.read_one().is_err());
    }
}