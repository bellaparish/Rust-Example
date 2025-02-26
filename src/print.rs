pub fn run() {
    //print to console
    println!("Hello from the print.rs function");

    //Basic Formatting
    println!("{} is from {}", "Bella", "Texas");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
    "Bella", "Texas", "be gay"
    );

    //Named Arguments
    println!("{name} likes to {activity}",
    name = "Bella",
    activity = "be gay"
    );

    //Placeholder Traits
    println!("Binary: {:b} Hex:{:x} Octal: {:o}",10,10,10);

    //Placeholder for debug trait
    println!("{:?}",(12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}",10+10);

}