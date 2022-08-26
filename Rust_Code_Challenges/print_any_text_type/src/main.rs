//fn info<T: std::fmt::Display>(a: &T) {
//    println!("{}",a);
//}

// allocates memory
//fn info<T: ToString>(a: &T) {
//    println!("{}",a.to_string());
//}

// Does not allocates memory
fn info<T: AsRef<str>>(a: &T) {
    println!("{}",a.as_ref());
}

fn main() {
    let a = "?";
    let b = "Hello, World!".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    // use std::ffi::CString;
    
    // let c = CString::new("?").unwrap();
    // info(&input);

    // Advanced 2
    // use std::path::Path;
    // let d = Path::new("/tmp/linkedin-learning");
    // info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }
