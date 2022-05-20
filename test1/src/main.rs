fn main() {
    let s = String::from("hello");
    let mut v = "ever ";

    let c = &mut v;
    // let b = c;

    println!("{}", c);
    println!("{:?}", s.as_ptr());
    // println!("{}", s.as_ptr());
    println!("{}", s);
    println!("Hello, world!");

    let some:i32 = 1;
    // let some = match some {
    //     Ok(f) => f,
    //     Err(e) => panic!("some error {}", e)
    // };
}
