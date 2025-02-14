use std::io;

#[derive(Debug)] 
struct Point {
    x: f64,
    y: f64,
}

//check if the points are equal if equal call double_point function if not equal call add_points function
//in the add_points function first get lamba which is equal to the difference of the y values of the two points divided by the difference of the x values of the two points y2-y1/x2-x1
//then get the x value of the new point by squaring the lambda value and subtracting the x value of the first point and subtracting the x value of the second point lamba^2 - x1 - x2
//then get the y value of the new point by multiplying the lambda value by the difference of the x1 - x3(new point) and subtracting the y value of the first point lambda(x1 - x3(new point)) - y1
//return the new point
fn add_points(p1: &Point, p2: &Point) -> Point {
    let lambda = (p2.y - p1.y) / (p2.x - p1.x);
    let x3 = lambda.powi(2) - p1.x - p2.x;
    let y3 = lambda * (p1.x - x3) - p1.y;
    Point { x: x3, y: y3 }
}

//double the point by getting lambda = 3x^2 + a / 2y
//value of a takes from the elliptic curve equation y^2 = x^3 + ax + b
//then get the x value of the new point by squaring the lambda value and subtracting the x value of the first point and subtracting the x value of the second point lamba^2 - x1 - x2
//then get the y value of the new point by multiplying the lambda value by the difference of the x1 - x3(new point) and subtracting the y value of the first point lambda(x1 - x3(new point)) - y1
//return the new point

fn double_point(p: &Point, a: f64) -> Point {
    let lambda = (3.0 * p.x.powi(2) + a) / (2.0 * p.y);
    let x3 = lambda.powi(2) - 2.0 * p.x;
    let y3 = lambda * (p.x - x3) - p.y;
    Point { x: x3, y: y3 }
}

fn main() {
    let mut input = String::new();

    println!("Enter the coefficients a and b for the elliptic curve equation y^2 = x^3 + ax + b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coefficients: Vec<f64> = input.trim().split_whitespace()
                                      .map(|s| s.parse().expect("Please enter a valid number"))
                                      .collect();
    let a = coefficients[0];
    let _b = coefficients[1]; // Use underscore to avoid unused variable warning

    println!("Enter the coordinates for point p1 (x y):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coords: Vec<f64> = input.trim().split_whitespace()
                                .map(|s| s.parse().expect("Please enter a valid number"))
                                .collect();
    let p1 = Point { x: coords[0], y: coords[1] };

    println!("Enter the coordinates for point p2 (x y):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coords: Vec<f64> = input.trim().split_whitespace()
                                .map(|s| s.parse().expect("Please enter a valid number"))
                                .collect();
    let p2 = Point { x: coords[0], y: coords[1] };

    let p3 = add_points(&p1, &p2);
    let p4 = double_point(&p1, a);

    println!("Result of adding points: {:?}", p3);
    println!("Result of doubling point: {:?}", p4);
}