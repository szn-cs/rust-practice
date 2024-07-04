/**
 * using hashmap nodes
 */
pub mod impl_1 {
    use std::collections::HashMap;

    const SUFFIX_LETTER: char = '*'; // either through relying on enum or suffix letter

    pub enum Node {
        LETTER {
            value: char,
            children: HashMap<char, Box<Node>>,
        },
        SUFFIX(String), // represents end of word (stores the word for easy access)
    }

    pub struct Trie {
        root: Node,
    }
}
