// Vectors - extend arrays list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Reassign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Vector Value: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slices: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop & Mutate values
    for x in numbers.iter_mut() {
        *x  *= 2;
    }
    println!("{:?}", numbers);
}
