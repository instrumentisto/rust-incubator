#[derive(Clone, Copy, Default, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug)]
struct Polyline {
    // Head garants emptiness
    head: Point,
    tail: Vec<Point>,
}

fn main() {
    let point1 = Point {
        x: 1.0,
        ..Default::default()
    };
    let point2 = Point {
        y: 1.0,
        ..Default::default()
    };
    println!("Points: {:?} ; {:?}", point1, point2);
    let polyline = Polyline {
        head: point1,
        tail: vec![point2],
    };
    println!("Polyline from points: {:?}", polyline);
}
