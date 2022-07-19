// 


pub fn run(){
    let name = "Damola";
    let mut age = 20;

    println!("My name is {} and I am {}", name, age);

    age = 21;
    println!("My name is {} and I am {}", name, age);

    //Define Constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multple var
    let (my_name, my_age) = ("Damola", 20);
    println!("{} is {}", my_name, my_age);
}