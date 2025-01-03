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

pub fn parse(s: &str) -> Result<Vec<Op>, ParseError> {
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
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let actual = parse("+ + * - /").unwrap();
        assert_eq!(actual, [Add, Add, Mul, Sub, Div])
    }
}
