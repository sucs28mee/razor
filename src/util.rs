use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug, Clone, Copy)]
pub struct Span<T> {
    index: usize,
    len: usize,
    pub value: T,
}

impl<T> Span<T> {
    pub fn start(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn range(&self) -> Range<usize> {
        self.index..self.end()
    }

    pub fn end(&self) -> usize {
        self.index + self.len
    }

    pub fn map<E, F: FnOnce(T) -> E>(self, f: F) -> Span<E> {
        Span {
            index: self.index,
            len: self.len,
            value: f(self.value),
        }
    }
}

impl<T> Deref for Span<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Span<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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

pub fn map_spans<'a, T, I, F>(
    spans: impl IntoIterator<IntoIter = I>,
    str: &str,
    mut f: F,
) -> Option<String>
where
    T: 'a,
    I: DoubleEndedIterator<Item = &'a Span<T>>,
    F: FnMut(&str) -> String,
{
    let (res, index) = spans
        .into_iter()
        .try_fold((String::new(), 0), |(acc, index), span| {
            let (prev, fmt) = (str.get(index..span.start())?, str.get(span.range())?);
            Some((format!("{acc}{prev}{}", f(fmt)), span.end()))
        })?;

    Some(format!("{res}{}", str.get(index..)?))
}
