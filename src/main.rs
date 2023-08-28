use std::io;
use std::collections::HashMap; //import HashMap collection

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
                    println!("3. Press '3' to Delete User by ID");
                    println!("4. Press '4' to View User by ID");
                    println!("5. Press '5' to Update User Name");
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
                        "5"=> {
                            println!("");
                            updateuserbyid(&mut users); // Update users by ID
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
        add_user = add_user.trim().to_lowercase(); //Convert user input to lowercase

        match add_user.trim() { //Match user inputs
            "q" => { //if 'q' exit
                println!("");
                println!("Exiting from adding users....");
                println!("");
                break;
            }, //else add user ID
            _ => {
                let mut id = String::new();
                println!("Enter {}'s ID :", add_user);
                io::stdin().read_line(&mut id).expect("Not a valid string");
                println!("");
                
                if let Ok(idnum) = id.trim().parse::<u32>() { //Check user input is a number
                    if users.contains_key(&idnum) { //Check user name is already in the HashMap
                        println!("User ID already exists");
                        println!("Enter New User ID");
                        println!("");
                        continue;
                    } else { //Else add user to the HahMap
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

//View all users
fn viewallusers(users: &mut HashMap<u32, String>) {
    let mut i = 1;
    println!("Welcome to View All Users");

    if users.is_empty() { //Check user list is empty
        println!("!!!!!!!!!!!!!!!!!!!!!!!");
        println!("User List is Empty.");
        println!("Add users to view list.");
        println!("!!!!!!!!!!!!!!!!!!!!!!!");
    } else { //View users
        println!("");
        println!("--------------Users List-------------");

        let mut sorted_list: Vec<(&u32, &String)> = users.iter().collect(); //Sort HashMap and store it in vector
        sorted_list.sort_by_key(|a| a.0); //Sort HashMap by Key

        //Itarate the sorted_list vector using for loop
        for (id, name) in sorted_list {
            println!("{}. User ID: {} | User Name: {}", i, id, name); //print the lis
            i += 1;
        }
    }


}

//Delete user
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
        id = id.trim().to_lowercase(); //Conert user input to lowercase

        println!("");

        match id.trim() { //Match user input
            "q"=> { //id 'q' exiting from the deleting users
                println!("");
                println!("Exiting from deleting users by ID....");
                println!("");
                break;
            }, 
            _ => {//else
                if let Ok(uid) = id.trim().parse::<u32>() { //Check user ID is a number
                    if users.contains_key(&uid) { //Check user is in the HashMap
                        users.remove(&uid);//remove user
                        println!("User Deleted Successfully");
                        println!("");
                    } else { //else user not found in the HashMap
                        println!("");
                        println!("No user found in ID: {}", uid);
                        println!("");
                    }

                }else { //esle not a valid number
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                }
            }
        }
    }
    
}

//View user by ID
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
        id = id.trim().to_lowercase(); //Convert user input to the lowercase

        println!("");

        match id.trim() { //Match user inputs
            "q"=> { //If user input is 'q' exit
                println!("");
                println!("Exiting from viewing users by ID....");
                println!("");
                break;
            }, 
            _ => {
                if let Ok(uid) = id.trim().parse::<u32>() { //Check user input is a number
                    if users.contains_key(&uid) { //Check user ID is in the HashMap
                        let name = users.get(&uid).unwrap(); //get user ID
                        println!("ID: {} | Name: {}", uid, name); 
                        println!("");
                    } else { //Else user not found in the HashMap
                        println!("");
                        println!("No user found in ID: {}", uid);
                        println!("");
                    }

                }else { //else user input is not a number
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                }
            }
        }
    }
    
}

//Update user by ID 
fn updateuserbyid(users: &mut HashMap<u32, String>) {
    println!("");
    println!("Welcome to Update User By ID");
    println!("");

    loop {
        let mut id = String::new();
        println!("Enter user's ID to Update User Name | Press 'q' to Stop updating Users");
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read ID.");
        id = id.trim().to_lowercase(); //Convert user input to the lowercase

        println!("");

        match id.trim() { //Match user inputs
            "q"=> { //If user input is 'q' exit
                println!("");
                println!("Exiting from updating users by ID....");
                println!("");
                break;
            }, 
            _ => {
                if let Ok(uid) = id.trim().parse::<u32>() { //Check user input is a number
                    if users.contains_key(&uid) { //Check user ID is in the HashMap
                        let name = users.get(&uid).unwrap(); //get user ID
                        println!("Current: ID: {} | Name: {}", uid, name);
                        println!("Enter new user name:");
                        let mut new_name = String::new(); //New user name
                        io::stdin()
                            .read_line(&mut new_name)
                            .expect("Failed to read new user name");
                        new_name = new_name.trim().to_string();//Convert new user name to string
                        users.insert(uid, new_name); //Update user name in the HashMap
                        let updated_name = users.get(&uid).unwrap(); //get user ID
                        println!("Updated: ID: {} | Name to: {}", uid, updated_name);
                        println!("");
                    } else { //Else user not found in the HashMap
                        println!("");
                        println!("No user found in ID: {}", uid);
                        println!("");
                    }

                }else { //else user input is not a number
                    println!("");
                    println!("!!!---Not a valid number for user ID---!!!");
                    println!("");
                }
            }
        }
    }
}