enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> u8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}

fn main() {
    let triangle = Shape::Triangle;
    let square = Shape::Square;

    println!("{}", triangle.corners());
    println!("{}", square.corners());
}
