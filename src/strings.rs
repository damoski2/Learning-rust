pub fn run(){
    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    //push char
    hello.push('W');

    //Push string
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is Empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'World': {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    println!("{}", hello);
}