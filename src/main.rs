use std::io;
use std::collections::HashMap; //import HashMap Struct

fn main() {
    let mut users: HashMap<u32, String> = HashMap::new();

    loop {
        println!("Enter 's' to Start || Enter 'q' to Quit");
        let mut user_input = String::new(); //Get user input to start the program
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");
        user_input = user_input.trim().to_lowercase(); //convert to lowercase 

        //Check if user want's to exith
        if user_input == "q" {
            println!("");
            println!("Exiting....");
            println!("");
            break;
        }
    
    
        loop {
            match user_input.trim() {
                //if user want's to start program
                "s"=> {
                    println!("");
                    //Menu
                    println!("---------------Menu--------------");
                    println!("1. Press '1' to Add User");
                    println!("2. Press '2' to View Users");
                    println!("4. Press '3' to Delete User by ID");
                    println!("3. Press '4' to View User by ID");
                    println!("6. Press '0' to Exit");
                    println!("---------------------------------");
                    println!("");
    
                    let mut menu_input = String::new(); //Get user inputs for match the menu
                    io::stdin()
                        .read_line(&mut menu_input)
                        .expect("Failed to read user input");
                
    
                    //Match what user want's to do
                    match menu_input.trim() {
                        "1"=> {
                            println!("");
                            adduser(&mut users); //Add users
                        },
                        "2"=> {
                            println!("");
                            viewallusers(&mut users); //View all users
                            println!("");
                        },
                        "3"=> {
                            println!("");
                            deleteuserbyid(&mut users); // Delete users by ID
                            println!("");
                        },
                        "4"=> {
                            println!("");
                            viewuserbyid(&mut users); // View users by ID
                            println!("");
                        },
                        "0"=> { //Check if user wants to exit from menu
                            println!("");
                            println!("Exiting from menu....");
                            println!("");
                            break;
                        },
                        _=> { //Check if user enter an invalid input
                            println!("");
                            println!("Invalid input!!!");
                            println!("Try again.");
                            println!("");
                            continue;
                        },
                    }
    
    
                },
                _=> { //if user inputs values apart from 's' of 'q'
                    println!("");
                    println!("Invalid Input");
                    println!("");
                    break;
                }
            }
        }
    }
}


//Add user 
fn adduser(users: &mut HashMap<u32, String>) {
    
    //Define a loop to get user inputs continueously
    loop {
        println!("");
        println!("Enter user name | Press 'q' to exit from adding users");
        let mut add_user = String::new();
        io::stdin()
            .read_line(&mut add_user)
            .expect("Failed to read add user");
        add_user = add_user.trim().to_lowercase(); //

        match add_user.trim() {
            "q" => {
                println!("");
                println!("Exiting from adding users....");
                println!("");
                break;
            },
            _ => {
                let mut id = String::new();
                println!("Enter {}'s ID :", add_user);
                io::stdin().read_line(&mut id).expect("Not a valid string");
                println!("");
                
                if let Ok(idnum) = id.trim().parse::<u32>() {
                    if users.contains_key(&idnum) {
                        println!("User ID already exists");
                        println!("Enter New User ID");
                        println!("");
                        continue;
                    } else {
                        users.insert(idnum, add_user);
                    }
                    
                } else {
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                    continue;
                } 
            }
        }
    } 
}

fn viewallusers(users: &mut HashMap<u32, String>) {
    let mut i = 1;
    println!("Welcome to View All Users");

    if users.is_empty() {
        println!("!!!!!!!!!!!!!!!!!!!!!!!");
        println!("User List is Empty.");
        println!("Add users to view list.");
        println!("!!!!!!!!!!!!!!!!!!!!!!!");
    } else {
        println!("");
        println!("--------------Users List-------------");
        for (id, name) in users {
            println!("{}. User ID: {} | User Name: {}", i, id, name);
            i += 1;
        }
    }


}

fn deleteuserbyid(users: &mut HashMap<u32, String>) {

    println!("");
    println!("Welcome to Delete User By ID");
    println!("");

    loop {
        let mut id = String::new();
        println!("Enter user's ID | Press 'q' to Stop Deleting Users");
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read ID.");
        id = id.trim().to_lowercase();

        println!("");

        match id.trim() {
            "q"=> {
                println!("");
                println!("Exiting from deleting users by ID....");
                println!("");
                break;
            }, 
            _ => {
                if let Ok(uid) = id.trim().parse::<u32>() {
                    if users.contains_key(&uid) {
                        users.remove(&uid);
                        println!("User Deleted Successfully");
                        println!("");
                    } else {
                        println!("");
                        println!("No user found in ID: {}", uid);
                        println!("");
                    }

                }else {
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                }
            }
        }
    }
    
}

fn viewuserbyid(users: &mut HashMap<u32, String>) {

    println!("");
    println!("Welcome to View User By ID");
    println!("");

    loop {
        let mut id = String::new();
        println!("Enter user's ID to View | Press 'q' to Stop viewing Users");
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read ID.");
        id = id.trim().to_lowercase();

        println!("");

        match id.trim() {
            "q"=> {
                println!("");
                println!("Exiting from viewing users by ID....");
                println!("");
                break;
            }, 
            _ => {
                if let Ok(uid) = id.trim().parse::<u32>() {
                    if users.contains_key(&uid) {
                        let name = users.get(&uid).unwrap();
                        println!("ID: {} | Name: {}", uid, name);
                        println!("");
                    } else {
                        println!("");
                        println!("No user found in ID: {}", uid);
                        println!("");
                    }

                }else {
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                }
            }
        }
    }
    
}