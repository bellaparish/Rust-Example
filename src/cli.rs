pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "Bella";
    let status = "100%";

    //println!("Args: {:?}",command);

    if command == "Hello" {
        println!("Hello {}",name);
    } else if command == "Status" {
        println!("Status is {}",status)
    } else {
        println!("That is not a valid command!");
    }
}