use std::borrow::Borrow;

use std::io::{self, Read, Write};
use std::u8;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    // _phantom: std::marker::PhantomData<&'a u8>,
    key: &'a [u8],

    key_position: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            key_position: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|a| {
            *a ^= self.key[self.key_position];
            self.key_position = (self.key_position + 1) % self.key.len();
        });
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&mut self, data: Data) -> impl ExactSizeIterator<Item = u8> + 'b
    where
        Data: IntoIterator + 'b,
        <Data as IntoIterator>::Item: Borrow<u8>,
        <Data as IntoIterator>::IntoIter: ExactSizeIterator + 'b,
        'a: 'b,
    {
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        // std::iter::empty()

        let data = data.into_iter();
        let key = self.key;
        let mut key_position = self.key_position;
        self.key_position = (self.key_position + data.len()) % self.key.len();

        data.map(move |a| {
            let pos = key_position;
            key_position = (key_position + 1) % key.len();
            a.borrow() ^ key[pos]
        })
    }

    pub fn reader(self, rd: impl Read + 'a) -> impl Read + 'a {
        Reader { xor: self, rd }
    }

    pub fn writer(self, wr: impl Write + 'a) -> impl Write + 'a {
        Writer { xor: self, wr }
    }
}

struct Reader<'a, R: Read> {
    xor: Xorcism<'a>,
    rd: R,
}

impl<'a, R: Read> Read for Reader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.rd.read(buf)?;
        self.xor.munge_in_place(&mut buf[..n]);
        Ok(n)
    }
}

struct Writer<'a, W: Write> {
    xor: Xorcism<'a>,
    wr: W,
}

impl<'a, W: Write> Write for Writer<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf = self.xor.munge(buf).collect::<Vec<_>>();
        self.wr.write(&buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.wr.flush()
    }
}
