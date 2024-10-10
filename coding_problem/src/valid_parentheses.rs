use std::collections::{HashMap, HashSet};
use std::iter;

// stores parentheses type and its reverse
enum PAR {
    Open,
    Close,
}

const PAR_OPEN: [char; 3] = ['(', '[', '{'];
const PAR_CLOSE: [char; 3] = [')', ']', '}'];

pub fn validate_parentheses(s: &str) -> bool {
    let parentheses: Vec<(char, char)> = iter::zip(PAR_OPEN, PAR_CLOSE).collect();
    let par_set: HashSet<char> =
        HashSet::from_iter(parentheses.iter().flat_map(|&(c1, c2)| [c1, c2]));

    // filter non-parentheses
    let s: String = s.chars().filter(|c| par_set.contains(c)).collect();

    // match parentheses using a stack
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        // find character type
        let p: (PAR, usize) = {
            if let Some(p) = PAR_OPEN.iter().position(|&x| x == c) {
                (PAR::Open, p)
            } else if let Some(p) = PAR_CLOSE.iter().position(|&x| x == c) {
                (PAR::Close, p)
            } else {
                unreachable!("parentheses only expected.");
            }
        };

        // push/pop from stack
        match p {
            (PAR::Open, _) => {
                stack.push(c);
            }
            (PAR::Close, index) if stack.last() == Some(&PAR_OPEN[index]) => {
                stack.pop();
            }
            _ => return false,
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s = "(){}[]";
        let r = validate_parentheses(&s);

        assert_eq!(r, true);
    }

    #[test]
    fn case_2() {
        let s = "{()[((){}[]){}]()}";
        let r = validate_parentheses(&s);

        assert_eq!(r, true);
    }

    #[test]
    fn case_3() {
        let s = "(){}[]]";
        let r = validate_parentheses(&s);

        assert_eq!(r, false);
    }

    #[test]
    fn case_4() {
        let s = "(){(}[]";
        let r = validate_parentheses(&s);

        assert_eq!(r, false);
    }

    #[test]
    fn case_5() {
        let s = "(ead)a{wqer}[hgfdh]";
        let r = validate_parentheses(&s);

        assert_eq!(r, true);
    }
}
