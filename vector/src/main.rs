fn main() {
    let mut v = vec!["Apple", "Microsoft", "Google", "Facebook"];
    let m0 = v[0];
    v.push("Tesla");
    let m4 = v[4];
    // let m = match m {
    //     Some(name) => name,
    //     None => "",
    // };
    println!("# Item at 0 is: {}", m0);
    println!("# Item at 4 is: {}", m4);
    for (index, item) in v.iter().enumerate() {
        println!("Item at {} is: {}", index, item);
    }

}
