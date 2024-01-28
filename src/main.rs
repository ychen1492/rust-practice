use std::{cmp::Ordering,io};    // prelude
use rand::Rng; // trait

// fn main() {
//     println!("Guessing a number!");
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     // println!("Secret number is {}", secret_number);
//     loop{
//         println!("Input a number: ");
//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess)
//         .expect("Can't read");
//         // io::Result Ok, Err

//         // shadow
//         let guess:u32 = match guess.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("The number you give is: {}", guess);

//         match  guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!!"), // arm
//             Ordering::Greater => println!("Too large!!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             },
//         }

//     }
    
// }
const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;

    let x = x+1;

    let x = x*2;

    let mut number = 3;
    
    while number != 0{
        println!("{}", number);

        number = number - 1;
    }

    println!("The value of x is {}", x);

    let y = [0, 10, 23, 45, 34];

    for i in y{
        println!("the value is {}", i);
    }

    for num in (1..4).rev(){
        println!("the number is {}", num);
    }
    println!("Liftoff");

    let mut s = String::from("value");

    s.push_str(" is what");

    println!("{}", s);
} 
