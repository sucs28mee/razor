use std::{fmt::Debug, ops::Range};

#[derive(Debug)]
pub struct Span<T> {
    index: usize,
    len: usize,
    value: T,
}

impl<T> Span<T> {
    pub fn new(index: usize, len: usize, value: T) -> Self {
        Self { index, len, value }
    }

    pub fn one(index: usize, value: T) -> Self {
        Self {
            index,
            len: 1,
            value,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn range(&self) -> Range<usize> {
        self.index..self.index + self.len
    }
}

pub trait Spanned: Sized {
    fn spanned(self, index: usize, len: usize) -> Span<Self> {
        Span::new(index, len, self)
    }
}

impl<T> Spanned for T {}
