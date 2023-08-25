use std::io;

fn main() {
    
    let mut vec = Vec::new();

    println!("Enter 's' to start program!!!");
    println!("Enter 'q' to quit program!!!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Faild to read line");
    input = input.trim().to_string();
    let mut i = 1;
    let mut x = 1;

    loop {
        if input == "q" {
            println!("Quitting program...");
            break;
        }

        match input.trim() {
            "s" => {
                println!("Enter name {}: or press 'q' to stop | press 'v' to view list", i);
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read the line.");
                name = name.trim().to_string();

                if name == "q" {
                    break;
                }

                if name == "v" {
                    println!();
                    println!("List of Names");
                    println!("-------------");

                    if vec.is_empty() {
                        println!();
                        println!("No names in the list");
                        break;
                    }
                    for items in &vec {
                        println!("{}. {}", x, items);
                        x += 1;
                    }

                    println!();
                    continue;
                }

                vec.push(name);
                i += 1;
            }
            _ => {
                println!("Invalid input");
                break;
            }
        }
    }
}
