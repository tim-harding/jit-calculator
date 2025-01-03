#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
use Op::*;

#[derive(Debug, thiserror::Error)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let actual = Program::parse("+ + * - /").unwrap();
        assert_eq!(actual.0, [Add, Add, Mul, Sub, Div])
    }
}
