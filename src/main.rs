use std::io;

fn c_to_f(celsius: i32) -> i32 {
    celsius * (9/5) + 32
}
fn f_to_c(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) / (9/5)
}

fn read_line_string() -> String {
    let mut from = String::new();
    loop {
        println!("Convert from C or F?");
        io::stdin().read_line(&mut from)
            .expect("Error");
        from = String::from(from.trim());
        if from.len() == 1 && (from.to_lowercase() == "c" || from.to_lowercase() == "f") {
            return from
        }
        from = String::new();
        println!("Not valid conversion!");
    }
}
fn read_line_number() -> i32 {
    let mut input = String::new();
    let temp: i32;
    loop {
        println!("Value to convert: ");
        io::stdin().read_line(&mut input)
            .expect("Error");
        temp = match input.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Invalid value!");
                continue;
            },
        };
        return temp
    }
}
fn evaluate_result(from: &str, value: i32) -> i32 {
    if from.to_lowercase() == "c" {
        c_to_f(value)
    } else {
        f_to_c(value)
    }
}

fn main() {
    let from = read_line_string();
    let temp = read_line_number();
    let result = evaluate_result(&from, temp);

    println!("Result: {}", result);
}
