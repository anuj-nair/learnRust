fn main() {
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
    }
    println!("inner_planet is {}", inner_planet);
}