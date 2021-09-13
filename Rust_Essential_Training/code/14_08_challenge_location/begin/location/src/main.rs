/* YOUR CODE GOES HERE */
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}
impl Location {
    fn display(&self) {
        match &self {
            Location::Unknown => println!("Location Unknown"),
            Location::Anonymous => println!("Location Anonymous"),
            Location::Known(lat, long) => println!("Address is ({},{})", lat, long),
        }
    }
}
fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
