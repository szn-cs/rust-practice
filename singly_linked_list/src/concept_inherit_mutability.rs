pub struct S {
    pub text: String,
    secret: String,
}

pub fn inherited_mutability_mutable_reference() {
    let mut s: S = S {
        text: "hellow".into(),
        secret: "key".into(),
    };

    s.text = "new greetings".into();

    let t: &mut String = &mut s.text; // mutable reference to a field of a mutable struct.
    *t = "new new greetings".into();
}

pub fn inherited_mutability_shared_reference() {
    let s: S = S {
        text: "x".into(),
        secret: "y".into(),
    };

    let mut t: &mut String = &mut s.text; // compiler error: field must abide by borrowing rules.
    *t = "new".into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iherited_mutability() {
        inherited_mutability_mutable_reference();
    }
}
