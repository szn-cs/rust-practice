/**
 * BF recursive with selective branching: for each position of the string choose whether to pick left or right paranthesis.
 * O(2^n)T leaning towards O(n)T with proper math; O(2^n)S
 *
 */
pub fn generate_parenthesis(n: usize) -> Vec<String> {
    return generate_parenthesis_recursive(n, 0, 0, String::with_capacity(2 * n));
}

fn generate_parenthesis_recursive(n: usize, left: usize, right: usize, s: String) -> Vec<String> {
    if left + right == 2 * n {
        return [s].to_vec();
    }

    // open parenthesis
    let s1 = if left < n {
        let mut s = s.clone();
        s.push('(');
        generate_parenthesis_recursive(n, left + 1, right, s)
    } else {
        vec![]
    };

    // close parenthesis
    let s2 = if right < left {
        let mut s = s.clone();
        s.push(')');
        generate_parenthesis_recursive(n, left, right + 1, s)
    } else {
        vec![]
    };

    [s1, s2].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let r: Vec<String> = generate_parenthesis(3);
        assert_eq!(r, ["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
