pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Square {
    x: f32,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        (self.x.powi(2)).into()
    }
}

pub struct Circle {
    r: f32,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        (self.r.powi(2) * std::f32::consts::PI).into()
    }
}

pub struct Rectangle {
    x: f32,
    y: f32,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        (self.x * self.y).into()
    }
}

pub fn get_area_of<T: HasArea>(shape: T) -> f64 {
    shape.area()
}

#[test]
fn test() {
    let mut area = get_area_of(Square { x: 2.0 });
    println!("{}", area);
    area = get_area_of(Circle { r: 2.0 });
    println!("{}", area);
    area = get_area_of(Rectangle { x: 2.0, y: 3.2 });
    println!("{}", area);
}
