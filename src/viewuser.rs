use std::io;

fn main() {
  
    println!("Enter 's' to start | Enter 'q' to quit.");
    let mut i = 1;
    let mut j = 1;
    let mut vec = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Faild to read the line.");

    input = input.trim().to_string();

    loop {
      if input == "q" {
        println!("Exiting...");
        break;
      }
  
      match input.trim() {
        "s" => {
          println!("Enter name {}: | Enter 'q' to exit | Enter 'v' to view the list ", i);
          let mut next = String::new();
          io::stdin()
            .read_line(&mut next)
            .expect("Faild to read the line.");
          next = next.trim().to_string();

          if next == "q" {
            println!("Exiting...");
            break;
          }

          if next == "v" {
            println!("");
            println!("Name List");
            println!("---------");

            if vec.is_empty() {
              println!("Name List is Empty");
              continue;
            }
            for names in &vec {
              println!("{}. {}", j, names);
              j += 1;
            }
            println!("");
          }

          vec.push(next);
          i +=1;
        }
        _=> {
          println!("Invalid input")
        }
      }
    }
}

