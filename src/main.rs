use std::io;

fn main(){

    //Declare a vecter
    let mut name_vec = Vec::new();

    loop{
        println!("Enter 's' to Start | Enter 'q' to Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        
        input = input.trim().to_string();

        checknumber(&input);

        if input == "q" { //if user enter 'q'
            println!("");
            println!("Exit....");
            println!("");
            break; //Stop running
        }

        addname(input, &mut name_vec); //Call add users function
    }

}

//Check number
fn checknumber(input: &String){
    if let Ok(..) = input.trim().parse::<i32>() {
         println!("");
         println!("You enterd an Number");
    }   
}

//Add users
fn addname(input: String, name_vec: &mut Vec<String>){

    loop {
        match input.trim() {
            "s" => {
                println!("Enter name {} | Enter 'n' to stop add | Enter 'v' view name list" );
                let mut names = String::new();
                io::stdin()
                    .read_line(&mut names)
                    .expect("Failed to read name");
                names = names.trim().to_string();

                println!("");

                if names == "n" {
                    println!("Exit from add");
                    println!("");
                    break;
                }

                if names == "v" {
                    viewlist(name_vec);
                    continue;
                }

                name_vec.push(names);
                
            }
            _=> {
                println!("Invalid input");
                println!("");
                break;
            }
        }
    }
}

//View all users
fn viewlist(name_vec: &mut Vec<String>){

    let mut j = 1;

    if name_vec.is_empty(){
        println!("Name list is empty.");
    }

    println!("");
    println!("Name List");
    println!("-----------");

    for names in name_vec {
        println!("{}. {}", j, names);
        j += 1;
    }

    println!("")
}