use std::{fmt::Debug, ops::Range};

#[derive(Debug)]
pub struct Span<T> {
    index: usize,
    len: usize,
    pub value: T,
}

impl<T> Span<T> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn range(&self) -> Range<usize> {
        self.index..self.index + self.len
    }

    pub fn map<E, F: FnOnce(T) -> E>(self, f: F) -> Span<E> {
        Span {
            index: self.index,
            len: self.len,
            value: f(self.value),
        }
    }
}

impl<T, E> From<Span<Result<T, E>>> for Result<Span<T>, Span<E>> {
    fn from(span: Span<Result<T, E>>) -> Self {
        match span.value {
            Ok(value) => Ok(value.spanned(span.index, span.len)),
            Err(err) => Err(err.spanned(span.index, span.len)),
        }
    }
}

pub trait Spanned: Sized {
    fn spanned(self, index: usize, len: usize) -> Span<Self> {
        Span {
            index,
            len,
            value: self,
        }
    }

    fn spanned_one(self, index: usize) -> Span<Self> {
        self.spanned(index, 1)
    }
}

impl<T> Spanned for T {}
