fn main() {
    let word = String::from("first");
    let word2 = String::from("apple");
    
    println!("{}", pig_latin(&word));
    println!("{}", pig_latin(&word2));
}

//스트링을 피그 라틴(pig Latin)으로 변경해보세요. 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 
// “ay”를 붙이므로, “first”는 “irst-fay”가 됩니다. 모음으로 시작하는 단어는 대신 끝에 “hay”를 붙입니다. 
// (“apple”은 “apple-hay”가 됩니다.) UTF-8 인코딩에 대해 기억하세요!
fn pig_latin(str: &str) -> String {
    let bytes = str.as_bytes();
    let mut first_alphabet: &str = &String::from("D")[..];
    let mut remain_word: &str = &String::from("efault")[..];
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            first_alphabet = &str[0..i+1];
            remain_word = &str[i+1..];
            break;
        }
    }

    let arr = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    for v in arr.iter() {
        if first_alphabet == &v[..] {
            let re = format!("{}{}{}", first_alphabet, remain_word, "-hay");
            return re;
        }
    }
    let re = format!("{}-{}{}", remain_word, first_alphabet, "ay");
    re
}
