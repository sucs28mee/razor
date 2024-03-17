use std::{
    fmt::Debug,
    iter::Peekable,
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug, Clone, Copy)]
pub struct Span<T> {
    pub start: usize,
    pub end: usize,
    pub value: T,
}

impl<T> Span<T> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
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

pub trait Spanned: Sized {
    fn spanned(self, range: Range<usize>) -> Span<Self> {
        Span {
            start: range.start,
            end: range.end,
            value: self,
        }
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
            Some((format!("{acc}{prev}{}", f(fmt)), span.end))
        })?;

    Some(format!("{res}{}", str.get(index..)?))
}
