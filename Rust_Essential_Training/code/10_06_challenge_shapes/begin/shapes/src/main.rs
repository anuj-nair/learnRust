/* YOUR CODE GOES HERE */
struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
    fn new(width: f64, length: f64) -> Rectangle {
        Rectangle {
            width: width,
            length: length,
        }
    }
    fn get_area(&self) -> f64 {
        self.width * self.length
    }

    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.length *= scale;
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
