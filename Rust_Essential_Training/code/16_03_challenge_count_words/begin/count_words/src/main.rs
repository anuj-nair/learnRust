// args ../earth_to_the_moon.txt

use std::collections::HashMap;
use std::env;
use std::fs;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

fn main() {
    if env::args().len() < 2 {
        println!("Usage:\n cargo run <path/to/the/file>");
    }

    let filename = env::args().nth(1).unwrap();
    println!("{:}", filename);

    let data = fs::read_to_string(filename);
    let data = match data {
        Ok(val) => val,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let data = data.trim().replace("\n", "").replace("\r", "");
    println!("{}", data);
    let words = data.split(" ");

    let mut word_count = HashMap::new();
    for word in words {
        if word.trim() != "" {
            let val = word_count.entry(word).or_insert(0);
            *val += 1
        }
    }
    println!("{:?}", word_count);
}
