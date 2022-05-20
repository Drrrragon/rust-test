use std::io::{stdin, stdout, Write};
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let some_int: u32 = rand::thread_rng().gen_range(0..100);
    let mut user_answer: String = String::new();
    
    // let mut user_answer: u32 = 0;
    loop {
        user_answer.clear();
        print!("enter your answer: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut user_answer).expect("failed to read for some reasion");
        let user_answer: u32 = match user_answer.trim().parse() {
            Ok(i) => i,
            Err(..) => {
                println!("worng integer");
                continue;
            }
        };
        // Old
        // if user_answer == some_int {
        //     println!("u won");
        //     break;
        // } else if user_answer > some_int {
        //     println!("smaller then u think");
        // } else {
        //     println!("bigger then u think");
        // }
        match user_answer.cmp(&some_int) {
            Ordering::Less => println!("bigger then u think"),
            Ordering::Greater => println!("smaller then u think"),
            Ordering::Equal => {
                println!("u won");
                break;
            }
        };
    }
}
