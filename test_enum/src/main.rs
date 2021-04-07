use std::collections::HashMap;

//#[derive(PartialEq, Eq, Hash)]
enum Key {
    First,
    Second,
    Third,
}

fn main () {
    let mut map:  HashMap<Key, String> = HashMap::new();
    map.entry(Key::First).or_insert("first".to_string());
    map.insert(Key::Second, "second".to_string());
    map.insert(Key::Third, "third".to_string());

    let val_str = map.get(&Key::Second).unwrap();
    assert_eq!(&"second", val_str);
}
