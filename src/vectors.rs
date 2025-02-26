// Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    println!("{:?}",numbers);

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}",numbers);

    // Add to vector
    numbers.push(5);
    numbers.push(6);

    println!("{:?}",numbers);

    // Pop from vector
    numbers.pop();

    println!("{:?}",numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Vector Length: {}",numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}",x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec {:?}",numbers);
}