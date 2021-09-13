use std::env;
use std::fs;


fn main() {
    if env::args().len() <= 2{
        println!("Program requires as least 2 arguments.");
        return;
    }


    let file = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    println!("File is {}",file);
    println!("Name is {}",name);

    let contents = fs::read_to_string(&file).unwrap();

//    let file= env::args().nth(1).unwrap();

    for line in contents.lines(){
        if line == name {
            println!("{} in {}", name, file);
            return;
        }
    }


    println!("{} Not in {}", name, file);
}
