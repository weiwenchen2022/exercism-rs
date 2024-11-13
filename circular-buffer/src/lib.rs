use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    // phantom: std::marker::PhantomData<T>,
    elements: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.elements.capacity() == self.elements.len() {
            return Err(Error::FullBuffer);
        }

        self.elements.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.elements.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.elements.capacity() == self.elements.len() {
            self.elements.pop_front();
        }
        self.elements.push_back(element);
    }
}
