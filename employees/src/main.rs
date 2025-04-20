use std::{io, collections::HashMap};

fn main() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names
    // to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company
    // by department, sorted alphabetically.

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome to the Employee database!\n");
    println!("Available commands:");
    println!("* add <name> to <department>");
    println!("* list [<department>]");
    println!("* exit");

    loop {
        println!("\nEnter command");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let mut iter = command.split_ascii_whitespace();

        match iter.next() {
            Some("add") => {
                let name = iter.next();
                let _to = iter.next();
                let department = iter.next();
                match (name, department) {
                    (Some(name), Some(department)) => {
                        employees.entry(department.to_string())
                            .or_insert_with(Vec::new)
                            .push(name.to_string());
                        println!("Added Employee '{}' to '{}'", name, department);
                    },
                    _ => println!("Error: could not parse, expected 'add <name> to <dept>'"),
                }
            },
            Some("list") => {
                let department = iter.next();
                match department {
                    // List only one specific department
                    Some(department) => {
                        match employees.get_mut(&department.to_string()) {
                            Some(names) => {
                                names.sort();
                                for name in names {
                                    println!("  {}", name);
                                }
                            },
                            None => println!("Error: could not find department '{}'", department),
                        }
                    },
                    // List all departments
                    _ => {
                        for (department, employees) in employees.iter_mut() {
                            println!("\n{}:", department);
                            employees.sort();
                            for name in employees {
                                println!("  {}", name);
                            }
                        }
                    },
                }
            },
            Some("exit") => break,
            _ => println!("Unrecognized command"),
        }
    }
}
