use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    //int_list();
    //pig_latin();
    employee();
}

#[allow(dead_code)]
fn int_list() {
    println!("----------------------");
    let mut int_list = vec![
        12, 1, 2, 3, 1, 6, 19, 40, 45, 30, 12, 23, 10, 12, 2, 1, 6, 5, 6, 29, 40,
    ];

    let mut sum = 0.0;
    for &item in &int_list {
        sum = item as f64 + sum;
    }
    let len: f64 = int_list.len() as f64;
    let the_mean = sum / len;
    println!("The mean: {}/{}={}", sum, len, the_mean);

    &int_list.sort();
    println!("List sorted: {:?}", int_list);

    let mid_index = int_list.len() / 2;
    let mid_item = int_list[mid_index];
    println!("The median item: {}", mid_item);

    let mut mode_list: HashMap<i32, i32> = HashMap::new();
    for &item in &int_list {
        let count = mode_list.entry(item).or_insert(0);
        *count += 1;
    }

    println!("The mode: {:#?}", mode_list);
}

#[allow(dead_code)]
fn pig_latin() {
    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("{}", answer);
    let vowels = "aeiou";
    let mut vowels = vowels.chars();
    loop {
        println!("Enter your word");

        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Can not read you input");
        let word = word.trim();
        let first_char = word
            .trim()
            .bytes()
            .next()
            .expect("Error during read your word");

        let is_first_char_vowel = vowels.any(|c| c as u8 == first_char);
        if is_first_char_vowel {
            println!("{}-hay", word)
        } else {
            let remain = &word[1..].trim();

            println!("{}-{}ay", remain, first_char as char);
        }
    }
}
#[derive(Eq, PartialEq, Hash)]
enum Department {
    Engineering = 1,
    Sales = 2,
    IT = 3,
}

impl Department {
    fn from_u8(value: u8) -> Department {
        match value {
            1 => Department::Engineering,
            2 => Department::Sales,
            3 => Department::IT,
            _ => panic!("Unknown value {}", value),
        }
    }
}

fn employee() {
    let mut employee: HashMap<Department, String> = HashMap::new();
    println!("1. Add employee");
    println!("2. Listing employee in a department");
    println!("3. Listing all employee in all department");
    println!("Please select an option: ");
    loop {
        let mut mode = String::new();
        io::stdin().read_line(&mut mode);
        let mode: u8 = match mode.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("{} is not supported", mode.trim());
                continue;
            }
        };
        if mode == 1 {
            input_mode(&mut employee);
        }
    }
}

fn input_mode(employee: &mut HashMap<Department, String>) {
    loop {
        println!("1. Engineering");
        println!("2. Sales");
        println!("3. IT");
        println!("0. Back");
        print!("Please select a department: ");
        io::stdout().flush().unwrap();
        let mut d = String::new();
        io::stdin()
            .read_line(&mut d)
            .expect("Can not process your input");
        let d: u8 = match d.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("You entered a string, please enter a number instead");
                continue;
                //eprintln!(err);
            }
        };
        if d > 3 || d < 0 {
            println!(
                "{} is not a valid department. Please select a department or 0 to exit",
                d
            );
            continue;
        }
        if d == 0 {
            break;
        }
        loop {
            let d = Department::from_u8(d);
            print!("Employee name (or 0 to go back): ");
            io::stdout().flush().unwrap();
            let mut employee_name = String::new();
            io::stdin()
                .read_line(&mut employee_name)
                .expect("Can not process your input");
            let employee_name = employee_name.trim();
            match employee_name.cmp(&"0") {
                Ordering::Equal => {
                    break;
                }
                Ordering::Less | Ordering::Greater => (),
            }
            employee.entry(d).or_insert(employee_name.to_string());
        }

    }
}
