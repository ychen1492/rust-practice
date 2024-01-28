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

    println!("The value of x is {}", x);
}
