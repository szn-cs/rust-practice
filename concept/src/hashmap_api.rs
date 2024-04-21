use std::collections::HashMap;
use std::option::Option::{None, Some};

pub fn hashmap_api() {
    {
        let c = HashMap::from([(1, "string 1"), (2, "string 2"), (3, "string 3")]); // not guaranteed to be sorted

        for (k, v) in c.iter() {
            println!("key: {}, value: {}", k, v);
        }

        for (k, v) in c.clone().into_iter() {
            println!("key: {}, value: {}", k, v);
        }

        for (k, v) in c.iter() {
            println!("key: {}, value: {}", k, v);
        }
    }

    {
        let mut c: HashMap<&str, &str> = HashMap::new();

        c.insert("key1", "value1");
        c.insert("key2", "value2");
        c.insert("key3", "value3");

        if let Some(v) = c.get("key2") {
            println!("value is {v}");
        } else {
            println!("key doesn't exist.")
        }

        println!("{:?}", c.get_key_value("key1"));

        let d = String::default();
        let d = d.as_str();

        let a = ["k1", "key1", "k2", "key3"];
        for k in a {
            match c.get(k) {
                Some(v) => {
                    println!("key: {k} exists with value {v}");
                }
                None => {
                    c.insert(k, d); // c lifetime doesn't exceed d lifetime
                }
            }
        }
    }
}
