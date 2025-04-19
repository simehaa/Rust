use std::io;

fn main() {
    let mut temp_unit = String::new();

    println!("Enter temperature unit (C or F):");
    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Failed to read line");
    temp_unit = temp_unit.trim().to_string();

    let mut temp_value = String::new();
    println!("Enter temperature value:");
    io::stdin()
        .read_line(&mut temp_value)
        .expect("Failed to read line");

    let temp_value: f32 = match temp_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        },
    };

    println!("Temperature: {}°{}", temp_value, temp_unit);

    if temp_unit.trim() == "C" {
        println!("Converted: {}°F", c_to_f(temp_value));
    } else if temp_unit.trim() == "F" {
        println!("Converted: {}°C", f_to_c(temp_value));
    } else {
        println!("Unknown unit: {}", temp_unit);
    }
}

fn c_to_f(temp_value: f32) -> f32 {
    temp_value * 9.0 / 5.0 + 32.0
}

fn f_to_c(temp_value: f32) -> f32 {
    (temp_value - 32.0) * 5.0 / 9.0
}
