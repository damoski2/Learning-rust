//Arrays - Fixed list where

use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    //Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get Array length
    println!("Array Length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..2];

    println!("Slice: {:?}", slice);

}