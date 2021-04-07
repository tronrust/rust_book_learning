fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut x: u8 = 255;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup = (10, 30, "the cat beat the dog");
    let (_a, b, _c) = tup;
    println!("The value of b in tub is: {}", b);
    println!("The value of c in tub is: {}", tup.2);

    let total = add_numbers(10, 5);
    println!("The result of add_numbers function is: {}", total);

    let number = if 3 > 5 { "yes" } else { "no" };
    println!("The if condition result: {}", number);

    //
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result is: {}", result);

    let array = [1, 20, 33, 490, 3];
    for element in array.iter() {
        println!("{}", element);
    }
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
