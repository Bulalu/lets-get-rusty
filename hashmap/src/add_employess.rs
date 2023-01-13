use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g. 'Add Sally to Engineering' or 'List Engineering'):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "Quit" {
            break;
        }

        let mut command = input.split_whitespace();
        let action = command.next().unwrap();
        let name = command.next().unwrap();
        
        let department = command.next().unwrap();

        match action {
            "Add" => {
                let names = company.entry(department.to_string()).or_insert(Vec::new());
                names.push(name.to_string());
                names.sort();
                println!("{} added to {}", name, department);
            }
            "List" => {
                match company.get(department) {
                    Some(names) => {
                        for name in names {
                            println!("{}", name);
                        }
                    }
                    None => println!("No such department"),
                }
            }
            _ => println!("Invalid command"),
        }
    }
}
