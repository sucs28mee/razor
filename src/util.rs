use std::fmt::Debug;

#[derive(Debug)]
pub struct Spanned<T> {
    pub index: usize,
    pub len: usize,
    pub value: T,
}

impl<T> Spanned<T> {
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
}
