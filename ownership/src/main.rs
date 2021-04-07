fn main() {
    let s = String::from("Hello");
    takes_ownership(s);

    let x = 5;
    take_copy(x);

    let nd = no_dangle();
    println!("No dangle says: {} ", nd);
    //takes_ownership(nd);
    //println!("No dangle says again: {} ", nd);

    let input = String::from("nguyen hong");
    let s = "nguyen hong tron";

    let first_word = first_word(&input[..]);
    //input.clear();
    println!("{}", first_word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn take_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item  == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}
