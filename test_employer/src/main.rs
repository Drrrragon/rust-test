use std::io::{stdin, stdout, Write};

// 해쉬맵과 벡터를 이용하여, 사용자가 회사 내의 부서에 대한 피고용인 이름을 추가할 수 있도록 하는 텍스트 인터페이스를 만들어보세요.
// 예를들어 “Add Sally to Engineering”이나 “Add Amir to Sales” 같은 식으로요.
// 그후 사용자가 각 부서의 모든 사람들에 대한 리스트나 알파벳 순으로 정렬된 부서별 모든 사람에 대한 리스트를 조회할 수 있도록 해보세요.

struct Engineering {
    name: String
}

struct Sales {
    name: String
}

fn default_message() {
    println!("=== Department program ===");
    println!("Chooese your command");
    println!("1. Add");
    println!("2. Delete");
    println!("3. List");
    println!("4. Exit");
    print!("-> ");
    stdout().flush().unwrap();
}

fn employee_message() {
    println!("");
    println!("1. Engineering");
    println!("2. Sales");
    println!("3. Previous");
    print!("-> ");
    stdout().flush().unwrap();
}

fn add_employee_name() {
    println!("Enter employee name");
    print!("-> ");
    stdout().flush().unwrap();
}

fn read_input_str(str: &mut String, function: fn()) -> &mut String{
    str.clear();
    stdout().flush().unwrap();
    function();
    stdin().read_line(str).expect("Un expected Error");
    str
}

fn main() {
    let mut engineering_vec: Vec<Engineering> = Vec::new();
    engineering_vec.push(Engineering { name : String::from("Evan") });
    let mut sales_vec: Vec<Sales> = Vec::new();
    sales_vec.push(Sales { name : String::from("Alex") });
    let mut str_input: String = String::from("");
    let mut int_input: i32;

    loop {
        int_input = match read_input_str(&mut str_input, default_message).trim().parse() {
            Ok(i) => i,
            Err(..) => {println!("!!! Worng type !!!"); println!(""); continue}
        };

        match int_input {
            1 => {
                loop {
                    int_input = match read_input_str(&mut str_input, employee_message).trim().parse() {
                        Ok(i) => i,
                        Err(..) => {println!("!!! Worng type !!!"); println!(""); continue}
                    };
                    match int_input {
                        1 => {
                            let answer = read_input_str(&mut str_input, add_employee_name).trim();
                            println!("Add \"{}\" to Engineering", answer);
                            engineering_vec.push(Engineering { name : answer.to_string() });
                            println!("Done");
                            println!("");
                        },
                        2 => {
                            let answer = read_input_str(&mut str_input, add_employee_name).trim();
                            println!("Add \"{}\" to Sales", answer);
                            sales_vec.push(Sales { name : answer.to_string() });
                            println!("Done");
                            println!("");
                        }
                        3 => break,
                        _ => ()
                    }
                }
            },
            3 => {
                loop {
                    int_input = match read_input_str(&mut str_input, employee_message).trim().parse() {
                        Ok(i) => i,
                        Err(..) => {println!("!!! Worng type !!!"); println!(""); continue}
                    };
                    match int_input {
                        1 => {
                            println!("");
                            println!("Print Engineering");
                            for i in &engineering_vec {
                                println!("{:?}", i.name);
                            }
                            println!("");
                        },
                        2 => {
                            println!("");
                            println!("Print Sales");
                            for i in &sales_vec {
                                println!("{:?}", i.name);
                            }
                            println!("");
                        }
                        3 => break,
                        _ => ()
                    }
                }
            },
            4 => break,
            _ => ()
        }
    }
}
