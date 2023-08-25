use std::io;

fn main() {
    //Define vector
    let mut name_vec = Vec::new();

    let mut i = 1;
    let mut j = 1;

    //Get user input to start or quit
    println!("Enter 's' to start | Enter 'q' to quit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    loop {
        if input == "q" {
            println!("Exiting...");
            break;
        }

        match input.trim() {
            "s" => {
                println!("");
                println!("Enter name {} | Enter 'q' to quit | Enter 'v' to view all users", i);
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                name = name.trim().to_string();

                if name == "q" {
                    println!("Exiting...");
                    break;
                }

                if name == "v" {
                    if name_vec.is_empty(){
                        println!("");
                        println!("Name list is empty");
                        println!("");
                        continue;
                    }

                    println!("");
                    println!("Name List");
                    println!("-----------");
                    
                    for names in &name_vec {
                        println!("{}. {}", j, names);
                        j += 1;
                    } 
                    println!("");
                    continue; 
                }

                name_vec.push(name);
                i += 1;
            }
            _=> {
                println!("Invalid input");
                continue;
            }
        }
    }
}

fn addusers()