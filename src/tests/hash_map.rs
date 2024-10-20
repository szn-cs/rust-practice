mod impl1 {
    use datastructure::hash_map::hash_map_1::{HashMap, Hashable};
    use specification::datastructure::hash_map::minimal::HashMap as Spec;

    #[test]
    fn create_hash_map() {
        #[derive(Clone, PartialEq, Eq)]
        struct StringKey(&'static str);

        impl Hashable for StringKey {
            fn hash(&self) -> u64 {
                let mut code: u64 = 12345;
                for c in self.0.bytes() {
                    code = (code << 5).wrapping_add(code).wrapping_add(c.into());
                }
                code
            }
        }

        let mut l = HashMap::new();

        l.insert(StringKey("str1"), 10);
        l.insert(StringKey("str2"), 20);
        l.insert(StringKey("str3"), 30);

        assert_eq!(l.len, 3);

        let replace = l.insert(StringKey("str3"), 300);
        assert_eq!(replace, Some(30));

        let value = l.get_mut(&StringKey("str2"));
        assert_eq!(value, Some(&mut 20));
        *value.unwrap() = 200;
        assert_eq!(l.get_mut(&StringKey("str2")), Some(&mut 200));
    }
}
