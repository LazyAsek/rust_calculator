use std::io;


fn main() {
    let a : i32 = load_number("give your first number");
    let b : i32 = load_number("give your secound number");

    println!("these are your number {} and {}",a,b);
    
    println!("Choose your action ( + , - , * ,/ )");
    let mut action : String = String::new();
    io::stdin().read_line( &mut action)
    .expect("unloaded");

    if action.trim() == "+"{
        println!("this is yours {}",addition(a,b));
    }
    if action.trim() == "-"{
        println!("this is yours {}",substract(a,b));
    }
    if action.trim() == "*"{
        println!("this is yours {}",multiply(a, b));
    }
    if action.trim() == "/"{
        println!("this is yours {}",divide(a,b));
    }
}

fn load_number(text : &str) -> i32{
    println!("{}",text);
    let mut input: String  = String::new();
    io::stdin().read_line(&mut input)
    .expect("unloaded");

    let number : i32 = input.trim().parse()
    .expect("it's not numebr");

    return number;
}

fn addition (a : i32, b : i32) -> i32 {
    return a+b;
}

fn substract (a : i32 , b : i32) -> i32{
    return a-b;
}

fn divide (a : i32,b : i32) -> i32{
    
    return a / b;
}

fn multiply (a : i32, b :i32) -> i32 {
    return a * b;
}