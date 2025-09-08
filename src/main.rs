//std = standard library io = input/output library
use std::cmp::Ordering;
use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    let mut seconds = 3;
    println!("Guess the number");

    //generates a number from 1-100 (syntax: (start..=end))
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess :3");
    
    // mut means the variable is mutable (value can change)
    // per default variables are immutable (value cant change)
    // :: indicates it is associated func with the string, new creates new empty string
    let mut guess = String::new();

    //calles stdin func from io module
    io::stdin()
        //calls the read_line method, passing &mut guess as argument
        //& is a reference
        //references are immutable by default
        //.read_line also outputs result (an enum)
        .read_line(&mut guess)
        .expect("Failed to read line");

    //shadows the value of guess to a new value
    //.trim() is needed before a string can be converted to u32, can only contain numerical data
    //user must press enter for read_line to input guess which adds a newline character to string
    //if it's 5 it would say 5\n
    //.trim() eliminates whitespace before/after, resulting in 5
    //.parse() converts string to another type
    //: after guess will annotate the variable's type
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guess: {guess}");

    //match is made up of "arms", an arm consists of a pattern to match against
    //if it matches, it runs that line of code
    //cmp compares guess to secret_number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!!");
            println!("I will now kill myself :3");
            while seconds > 0{
                println!("{}", seconds);
                thread::sleep(Duration::from_secs(1));
                seconds -= 1;
            }
            println!("Goodbye!");
            break;
        }
    }
    }
}
