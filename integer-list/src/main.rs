use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
// use std::vec::sort;

// use std::vec;

fn main() {
    let mut rnd = rand::thread_rng();
    let arr: [u8;10] = [rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100), rnd.gen_range(0..100)];
    println!("{:?}", &arr);
    
    let mut tmp :u32= 0;
    for i in &arr {
        tmp += *i as u32;
    }
    println!("Average: {}", tmp/arr.len() as u32);

    let arr = bubble_sort(arr);
    println!("Sorted: {:?}", arr);
    println!("Median: {}", arr[arr.len()/2 -1]);

    let mut map = HashMap::<u8, u8>::new();
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("Map: {:?}", map);
    println!("mode: {:?}", find_mode(map));
}

fn bubble_sort(mut arr: [u8;10]) -> [u8;10]{
    for i in 0..arr.len() -2 {
        for j in 0..arr.len() -1 -i {
            match arr[j].cmp(&arr[j + 1]) {
                Ordering::Equal => (),
                Ordering::Less => (),
                Ordering::Greater => {
                    let tmp = arr[j + 1];
                    arr[j + 1] = arr[j];
                    arr[j] = tmp;
                }
            }
        }
    }
    arr
}

fn find_mode(map: HashMap::<u8, u8>) -> (u8, u8) {
    let mut max = 0;
    let mut tmp = 0;
    for (k, v) in map {
        if tmp == 0 {
            tmp = v; 
            max = k;
            continue;
        }
        if v > (tmp as u8){
            tmp = v;
            max = k;
        }
    }
    (max, tmp)
}
