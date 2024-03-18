use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    pub start: usize,
    pub end: usize,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub trait Span: Sized {
    /// Spans a value over some range.
    fn span(self, range: Range<usize>) -> Spanned<Self> {
        Spanned {
            start: range.start,
            end: range.end,
            value: self,
        }
    }
}

impl<T> Span for T {}

pub fn map_spans<'a, T, I, F>(
    spans: impl IntoIterator<IntoIter = I>,
    str: &str,
    mut f: F,
) -> Option<String>
where
    T: 'a,
    I: DoubleEndedIterator<Item = &'a Spanned<T>>,
    F: FnMut(&str) -> String,
{
    let (res, index) = spans
        .into_iter()
        .try_fold((String::new(), 0), |(acc, index), span| {
            let (prev, fmt) = (str.get(index..span.start)?, str.get(span.range())?);
            Some((format!("{acc}{prev}{}", f(fmt)), span.end))
        })?;

    Some(format!("{res}{}", str.get(index..)?))
}
