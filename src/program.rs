use crate::jit::jit;
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

    pub fn jit(&self) -> ProgramJit {
        ProgramJit(jit(self.iter()))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramJit(extern "C" fn(f64) -> f64);

impl ProgramJit {
    pub fn exec(&self, arg: f64) -> f64 {
        self.0(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aamsd() -> Program {
        Program::parse("+ + * - /").unwrap()
    }

    #[test]
    fn parses() {
        assert_eq!(aamsd().0, [Add, Add, Mul, Sub, Div])
    }

    #[test]
    fn interprets() {
        let p = aamsd();
        assert_eq!(p.interpret(0.0), 1.5);
        assert_eq!(p.interpret(1.0), 2.5);
    }

    #[test]
    fn execs() {
        let p = aamsd().jit();
        assert_eq!(p.exec(0.0), 1.5);
        assert_eq!(p.exec(1.0), 2.5);
    }
}
