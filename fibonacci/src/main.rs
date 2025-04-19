use std::io;

fn main() {
    println!("Hello! I can print the nth Fibonacci number");
    println!("Please enter a number:");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        },
    };

    let mut prevprev = 0;
    let mut prev = 1;
    let mut current = 1;
    for _i in 1..number+1 {
        println!("{}", current);
        current = prevprev + prev;
        prevprev = prev;
        prev = current;
    }
}
