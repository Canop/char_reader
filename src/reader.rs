use {
    crate::unicode,
    std::{
        io::{self, Read},
    },
};

/// A buffered reader able to read chars or lines without crashing
/// when the stream doesn't finish and/or doesn't contain newlines.
///
/// It's also able to avoid storying whole lines if you're only
/// interested in their beginning.
///
/// Bad UTF8 is reported as io::Error with kind InvalidData.
pub struct CharReader<R: Read> {
    src: R,
    buffer: Box<[u8]>,
    pos: usize,
    len: usize,
}

const DEFAULT_BUF_SIZE: usize = 5_000;

impl<R: Read> CharReader<R> {
    pub fn new(src: R) -> Self {
        let buf_size = DEFAULT_BUF_SIZE;
        let buffer = vec![0; buf_size].into_boxed_slice();
        // we might be abte to skip filling with 0 with some unsafe
        Self {
            src,
            buffer,
            pos: 0,
            len: 0,
        }
    }
    /// ensure there's at least one char in the buffer, and returns it with
    /// its size in bytes (or None if the underlying stream is finished).
    ///
    /// You probably don't need this function but next_char.
    pub fn load_char(&mut self) -> io::Result<Option<(char, usize)>> {
        if self.pos >= self.len {
            // buffer empty
            self.len = self.src.read(&mut self.buffer)?;
            if self.len == 0 {
                return Ok(None);
            }
            self.pos = 0;
        }
        let b = self.buffer[self.pos];
        let char_size = unicode::utf8_char_width(b);
        if self.pos + char_size > self.len {
            // there's not enough bytes in buffer
            // we start by moving what we have at the start of the buffer to make some room
            self.buffer.copy_within(self.pos..self.len, 0);
            self.len = self.len - self.pos;
            self.len += self.src.read(&mut self.buffer[self.len..])?;
            if self.len < char_size {
                // we may ignore one to 3 bytes not being correct UTF8 at the
                // very end of the stream (ie return None instead of an error)
                return Ok(None);
            }
            self.pos = 0;
        }
        let code_point = unicode::read_code_point(&self.buffer, self.pos, char_size);
        match std::char::from_u32(code_point) {
            Some(c) => Ok(Some((c, char_size))),
            None => Err(io::Error::new(io::ErrorKind::InvalidData, "Not UTF8"))
        }
    }
    /// read and return the next char, or NONE in case of EOF
    pub fn next_char(&mut self) -> io::Result<Option<char>> {
        Ok(match self.load_char()? {
            Some(cw) => {
                self.pos += cw.1;
                Some(cw.0)
            }
            None => None,
        })
    }
    /// return the next char, but doesn't advance the cursor
    pub fn peek_char(&mut self) -> io::Result<Option<char>> {
        self.load_char().map(|cw| cw.map(|cw| cw.0))
    }
    /// append the next line, if any, but with some protection against
    /// wild stream content:
    /// - don't store chars after the drop_after threshold
    /// - throw an error after the fail_after threshold
    ///
    /// Thresholds are in chars, not bytes nor cols nor graphemes.
    /// Only difference with next_line is that you pass (and may reuse)
    /// the string to fill.
    ///
    /// Return Ok(false) when there was no error but nothing to read
    /// (stream finished or paused).
    ///
    /// This function may return Ok(true) and not have written anything:
    /// it means there was an empty line (i.e. next char will be a CR or LF)
    pub fn read_line(
        &mut self,
        line: &mut String, // the line to append to
        drop_after: usize, // don't put in the string chars after that threshold
        fail_after: usize, // throw an error if there's no new line before that threshold
    ) -> io::Result<bool> {
        let mut chars_count = 0; // chars seen
        loop {
            match self.next_char() {
                Err(e) => {
                    return Err(e);
                }
                Ok(None) => {
                    return if chars_count > 0 {
                        Ok(true)
                    } else {
                        Ok(false)
                    };
                }
                Ok(Some(c)) => {
                    if c == '\r' {
                        if let Ok(Some(('\n', 1))) = self.load_char() {
                            // we consume the LF following the CR
                            self.pos += 1;
                        }
                        return Ok(true);
                    } else if c == '\n' {
                        return Ok(true);
                    } else if chars_count >= fail_after {
                        return Err(io::Error::new(io::ErrorKind::Other, "Line too long"));
                    } else if chars_count >= drop_after {
                        //debug!("dropping char {}", c);
                    } else {
                        line.push(c);
                    }
                    chars_count += 1;
                }
            }
        }
    }
    /// return the next line, if any, but with some protection against
    /// wild stream content:
    /// - don't store chars after the drop_after threshold
    /// - throw an error after the fail_after threshold
    ///
    /// Thresholds are in chars, not bytes nor cols nor graphemes.
    pub fn next_line(
        &mut self,
        drop_after: usize, // don't put in the string chars after that threshold
        fail_after: usize, // throw an error if there's no new line before that threshold
    ) -> io::Result<Option<String>> {
        let mut line = String::new();
        match self.read_line(&mut line, drop_after, fail_after) {
            Ok(true) => Ok(Some(line)),
            Ok(false) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
