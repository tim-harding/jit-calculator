use std::iter::Copied;
use Op::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("Unrecognized character {0}")]
pub struct ParseError(char);

pub struct Program(Vec<Op>);

impl Program {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        s.chars()
            .filter_map(|c| {
                Some(Ok(match c {
                    '+' => Add,
                    '-' => Sub,
                    '*' => Mul,
                    '/' => Div,
                    ' ' => return None,
                    c => return Some(Err(ParseError(c))),
                }))
            })
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl IntoIterator for Program {
    type Item = Op;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Program {
    type Item = Op;

    type IntoIter = Copied<std::slice::Iter<'a, Op>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let actual = Program::parse("+ + * - /").unwrap();
        assert_eq!(actual.0, [Add, Add, Mul, Sub, Div])
    }
}
