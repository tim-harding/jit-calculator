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

    pub fn interpret(&self, arg: f64) -> f64 {
        self.iter().fold(arg, |acc, op| match op {
            Add => acc + 1.0,
            Sub => acc - 1.0,
            Mul => acc * 2.0,
            Div => acc / 2.0,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Op> + '_ {
        self.into_iter()
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

    #[test]
    fn interprets() {
        let actual = Program::parse("+ + * - /").unwrap();
        assert_eq!(actual.interpret(0.0), 1.5);
        assert_eq!(actual.interpret(1.0), 2.5);
    }
}
