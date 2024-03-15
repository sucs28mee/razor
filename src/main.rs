use std::{
    env,
    fmt::Debug,
    fs,
    iter::{Enumerate, Peekable},
    ops::Range,
};

fn main() {
    let path = env::args().next().expect("Expected a path argument.");
    for token in Parser::new(&fs::read(path).expect("IO Error"))
        .collect::<Result<Vec<_>, _>>()
        .expect("Parser Error")
    {
        println!("{:?}", token);
    }
}

struct Spanned<T> {
    span: Range<usize>,
    value: T,
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}) {:?}", self.span, self.value)
    }
}

#[derive(Debug, Clone)]
enum Token {
    Ident(String),
    Assignment,
    Brace { open: bool, kind: BraceKind },
}

#[derive(Debug, Clone, Copy)]
enum BraceKind {
    Curly,
    Square,
    Smooth,
}

struct Parser<I>
where
    I: Iterator,
{
    idx: usize,
    bytes: Peekable<I>,
}

impl<'a, I> Parser<I>
where
    I: Iterator<Item = &'a u8>,
{
    pub fn new<B>(bytes: B) -> Self
    where
        B: IntoIterator<IntoIter = I>,
    {
        Self {
            idx: 0,
            bytes: bytes.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.bytes.peek().copied().copied()
    }

    fn next(&mut self) -> Option<u8> {
        self.idx += 1;
        self.bytes.next().copied()
    }

    fn next_if(&mut self, f: impl FnOnce(&u8) -> bool) -> Option<u8> {
        self.bytes.next_if(|byte| f(*byte)).copied()
    }
}

#[derive(Debug, Clone, Copy)]
enum ParseError {
    UnexpectedToken,
}

impl<'a, I> Iterator for Parser<I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = Result<Spanned<Token>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.peek()?.is_ascii_whitespace() {
            self.next();
        }

        let start = self.idx;
        Some(match self.peek()? {
            b'a'..=b'z' | b'A'..=b'Z' => {
                let bytes = (0..)
                    .map_while(|_| self.next_if(|byte| byte.is_ascii_alphanumeric()))
                    .collect::<Vec<_>>();

                Ok(Spanned {
                    span: start..self.idx,
                    value: Token::Ident(String::from_utf8_lossy(&bytes).to_string()),
                })
            }
            _ => Err(ParseError::UnexpectedToken),
        })
    }
}
