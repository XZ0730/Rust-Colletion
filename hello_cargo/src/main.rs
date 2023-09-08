use rand::Rng;
use std::io;
use std::cmp::Ordering;



fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().
    gen_range(1, 101);
    println!("secret_number is:{}",secret_number);
    
    loop {
        let mut guess = String::new();
    println!("input a number:");
    io::stdin().read_line(&mut guess).
    expect("error happend");

    println!("the guess number is:{}",guess);
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("error happen");
            continue;
        },
    };
    // expect("parse error:type u32");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("To small"),
        Ordering::Greater => println!("To large"),
        Ordering::Equal =>{
            println!("success");
            break;
        },
    }
}
}
