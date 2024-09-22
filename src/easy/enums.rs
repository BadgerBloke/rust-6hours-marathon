use std::f64::consts::PI;

pub enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

pub fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => PI * r * r,
    }
}
