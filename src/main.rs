fn main() {
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64),
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Rectangle(width, height) => width * height,
                Shape::Triangle(base, height) => (base * height) / 2.0,
            }
        }
    }

    let c = Shape::Circle(64.0);
    let r = Shape::Rectangle(5.0, 4.0);
    let t = Shape::Triangle(5.0, 2.0);
    println!("c area: {:.2}", c.area());
    println!("r area: {}", r.area());
    println!("t area: {}", t.area());

    // Solution
    // use std::f64::consts::PI;
    //
    // enum Shape {
    //     Circle(f64),
    //     Rectangle(f64, f64),
    //     Triangle(f64, f64),
    // }
    //
    // impl Shape {
    //     fn area(&self) -> f64 {
    //         match self {
    //             Shape::Circle(r) => PI * r * r,
    //             Shape::Rectangle(w, h) => w * h,
    //             Shape::Triangle(b, h) => 0.5 * b * h,
    //         }
    //     }
    // }
    //
    // fn main() {
    //     let shapes = [
    //         Shape::Circle(5.0),
    //         Shape::Rectangle(4.0, 6.0),
    //         Shape::Triangle(3.0, 8.0),
    //     ];
    //     for shape in &shapes {
    //         println!("Area: {:.2}", shape.area());
    //     }
    // }
}
