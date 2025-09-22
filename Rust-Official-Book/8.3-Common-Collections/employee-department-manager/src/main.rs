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

fn main() {
    println!("==========================================================");
    println!("Welcome to the Employee Department Manager!");
    println!("==========================================================");

    loop { 
        print_available_commands();
        println!("Please enter a command:");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        println!("\n");

        let command_vector: Vec<&str> = command.split_whitespace().collect();
        println!("{:?}", command_vector);

        match command_vector[0].to_lowercase().as_str() {
            "quit" | "exit" | "e" | "q" => {
                println!("Exiting the program. Thank you!");
                break;
            },
            "add" => {
                
            },
            "list" => {
                match command_vector[1].to_lowercase().as_str() {
                    "all" => {

                    },
                    _ => {

                    }
                }
            },
            _ => println!("Invalid command. Please try again.\n"),
        }
    }

}

