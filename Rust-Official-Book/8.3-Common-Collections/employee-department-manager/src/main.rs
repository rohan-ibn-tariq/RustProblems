use std::collections::HashMap;
use std::io;

fn print_available_commands() {
    println!("==========================================================");
    println!("Available commands:");
    println!("1. Add [Employee Name] to [Department]");
    println!("2. List [Department]");
    println!("3. List All");
    println!("4. Exit | Quit | E | Q");
    println!("==========================================================");
    println!("\n");
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let (first, rest) = word.split_at(1);
            format!("{}{}", first.to_uppercase(), rest.to_lowercase())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn listing_department(data_base: &mut HashMap<String, Vec<String>>, department_name: &str) {

    match data_base.get_mut(department_name) {
        Some(employees) => {
            if employees.is_empty() {
                println!("No employees found in the '{}' department. \n", department_name);
            } else {
                employees.sort();
                println!("Department: {}", department_name);
                for employee in employees {
                    println!("- {}", employee);
                }
                println!("\n");
            }
        },
        None => println!("Department '{}' does not exist.\n", department_name),
    }
}

fn display_all_departments(database: &mut HashMap<String, Vec<String>>) {
    if database.is_empty() {
        println!("No departments found.\n");
        return;
    }

    for employees in database.values_mut() {
        employees.sort();
    }

    let mut departments: Vec<&String> = database.keys().collect();
    departments.sort();

    for department in departments {
        println!("Department: {}", department);
        match database.get(department) {
            Some(employees) if !employees.is_empty() => {
                for employee in employees {
                    println!("- {}", employee);
                }
                println!("\n");
            },
            _ => println!("- No employees found."),
        }
    }
}

fn main() {
    println!("==========================================================");
    println!("Welcome to the Employee Department Manager!");
    println!("==========================================================");

    let mut data_base: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print_available_commands();
        println!("Please enter a command:");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        println!("\n");

        let command_vector: Vec<&str> = command.split_whitespace().collect();

        if command_vector.len() == 0 {
            println!("No command entered. Please try again.\n");
            continue;
        }

        match command_vector[0].to_lowercase().as_str() {
            "quit" | "exit" | "e" | "q" => {
                println!("Exiting the program. Thank you!");
                break;
            }
            "add" => {
                let rest = &command_vector[1..];

                if let Some(to_index) = rest.iter().position(|&word| word.to_lowercase() == "to") {
                    let employee_name = to_title_case(&rest[..to_index].join(" "));
                    let department_name = to_title_case(&rest[to_index + 1..].join(" "));

                    if employee_name.is_empty() {
                        println!("Employee name cannot be empty. Please try again.\n");
                        continue;
                    } else if department_name.is_empty() {
                        println!("Department name cannot be empty. Please try again.\n");
                        continue;
                    } else {
                        let employees = data_base
                            .entry(department_name.clone())
                            .or_insert_with(Vec::new);

                        if employees.contains(&employee_name) {
                            println!(
                                "{} is already in the {} department.\n",
                                employee_name, department_name
                            );
                            continue;
                        } else {
                            employees.push(employee_name.clone());
                            println!(
                                "Added {} to {} department successfully!",
                                employee_name, department_name
                            );
                        } 
                    }
                } else {
                    println!("Invalid command format(Missing to). Please use: Add [Employee Name] to [Department]\n");
                    continue;
                }
            }
            "list" => {
                if command_vector.len() < 2 {
                    println!("Invalid command format. Please use: List [Department] or List All\n");
                    continue;
                } else {
                    match command_vector[1].to_lowercase().as_str() {
                        "all" => {
                            display_all_departments(&mut data_base);
                            continue;
                        }
                        _ => {
                            let department_name = to_title_case(&command_vector[1..].join(" "));
                            listing_department(&mut data_base, &department_name);
                            continue;
                        }
                    }
                }
            }
            _ => println!("Invalid command. Please try again.\n"),
        }
    }
}
