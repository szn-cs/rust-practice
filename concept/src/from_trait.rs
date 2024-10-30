#[derive(PartialEq, Eq, Debug)]
enum Method {
    Post,
    Get,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "POST" => Method::Post,
            "GET" => Method::Get,
            _ => Method::Uninitialized,
        }
    }
}

#[test]
fn test_converstion() {
    let s = "POST 1.1 XYZ";
    let mut words = s.split(' ');
    let type_method = words.next().unwrap();
    let method: Method = type_method.into();
    let method = Method::from(type_method);
    assert_eq!(Method::Post, method);
}
