#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn dist_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point::new(0.0, 0.0);
    let point2 = Point::new(3.0, 4.0);

    println!("Distance between point1({:?}) and point2({:?}): {}", point1, point2, point1.dist_to(&point2));

    let point3 = Point::new(1.0, 1.0);
    let point4 = Point::new(4.0, 5.0);

    println!("Distance between point3({:?}) and point4({:?}): {}", point3, point4, point3.dist_to(&point4));
}