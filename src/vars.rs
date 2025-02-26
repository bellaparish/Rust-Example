//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Bella";
    let mut age = 19;
    println!("My name is {}, and I am {}",name,age);
    age = 20;
    println!("My name is {}, and I am {}",name,age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //Assign multiple variables
    let ( my_name, my_age ) = ("Bella", 19);
    println!("My name is {}, and I am {}",my_name,my_age);
}