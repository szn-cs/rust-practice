/**
 * using hashmap nodes
 */
pub mod impl_1 {
    use std::collections::HashMap;

    const SUFFIX_LETTER: char = '*';

    pub enum NODE {
        LETTER {
            value: char,
            children: HashMap<char, Box<NODE>>,
        },
        SUFFIX(), // represents end of word
    }
}
