use std::vec;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let number = 50;
    println!("Optimized fibo function");
    println!("{}", lets_fibo(number));
    println!("{:.2?}", now.elapsed());
    now = Instant::now();
    println!("Recursive fibo function");
    println!("{}", recursive_fibo(number));
    println!("{:.2?}", now.elapsed());
}

fn lets_fibo(number: u64) -> u64 {
    if number <= 2 {
        return 1;
    }
    let mut vector = vec![1, 1];
    let mut ptr: usize = vector.len().try_into().unwrap();
    let number: usize = number.try_into().unwrap();
    while ptr < number {
        let ptr_u: usize = ptr.try_into().unwrap();
        vector.push(*vector.get(ptr_u -1).unwrap() + *vector.get(ptr_u -2).unwrap());
        ptr += 1;
    }  
    return vector[vector.len()-1];
}

fn recursive_fibo(number: u64) -> u64 {
    if number <= 1 {
        return number;
    }
    return recursive_fibo(number - 1) + recursive_fibo(number - 2)
}
