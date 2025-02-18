use num_bigint::BigInt;
use std::collections::HashSet;
use num_traits::ToPrimitive;

// Previous Point, Curve, and Isomorphism structs remain the same...
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: BigInt,
    y: BigInt,
}

#[derive(Clone, Debug)]
struct Curve {
    a: BigInt,
    b: BigInt,
    field_order: BigInt,
}

#[derive(Clone, Debug)]
struct Isomorphism {
    u: BigInt,
    r: BigInt,
    s: BigInt,
    t: BigInt,
}

impl Curve {
    fn new(a: BigInt, b: BigInt, field_order: BigInt) -> Self {
        Curve { a, b, field_order }
    }

    fn contains_point(&self, point: &Point) -> bool {
        let left_side = (&point.y * &point.y) % &self.field_order;
        let right_side = ((&point.x * &point.x * &point.x) + 
                         (&self.a * &point.x) + 
                         &self.b) % &self.field_order;
        left_side == right_side
    }

    // Find all points on the curve
    fn find_all_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        
        for x in 0..self.field_order.to_i32().unwrap() {
            for y in 0..self.field_order.to_i32().unwrap() {
                let point = Point {
                    x: BigInt::from(x),
                    y: BigInt::from(y),
                };
                if self.contains_point(&point) {
                    points.insert(point);
                }
            }
        }
        points
    }

    fn display(&self) -> String {
        format!("E: y² = x³ + {}x + {} over F_{}", self.a, self.b, self.field_order)
    }
}

impl Isomorphism {
    fn new(u: BigInt, r: BigInt, s: BigInt, t: BigInt) -> Self {
        Isomorphism { u, r, s, t }
    }

    fn transform_point(&self, point: &Point, field_order: &BigInt) -> Point {
        let u2 = &self.u * &self.u;
        let u3 = &u2 * &self.u;
        
        let new_x = (&u2 * &point.x + &self.r) % field_order;
        let new_y = (&u3 * &point.y + &u2 * &self.s * &point.x + &self.t) % field_order;
        
        Point { x: new_x, y: new_y }
    }
}

fn main() {
    // Define two isomorphic curves over F_7
    let curve1 = Curve::new(
        BigInt::from(2),  // a1
        BigInt::from(4),  // b1
        BigInt::from(7)   // p
    );

    // Define isomorphism parameters
    let iso = Isomorphism::new(
        BigInt::from(2),  // u
        BigInt::from(1),  // r
        BigInt::from(0),  // s
        BigInt::from(0)   // t
    );

    // Find all points on curve1
    let points1 = curve1.find_all_points();

    // Transform curve1 to get curve2
    let u2 = &iso.u * &iso.u;
    let u4 = &u2 * &u2;
    let u6 = &u4 * &u2;
    
    let a2 = (&u4 * &curve1.a) % &curve1.field_order;
    let b2 = (&u6 * &curve1.b) % &curve1.field_order;
    
    let curve2 = Curve::new(a2, b2, curve1.field_order.clone());

    // Find all points on curve2
    let _points2 = curve2.find_all_points();

    // Display the curves
    println!("Curve 1: {}", curve1.display());
    println!("Curve 2: {}", curve2.display());
    
    println!("\nPoints on Curve 1:");
    for point in &points1 {
        println!("({}, {})", point.x, point.y);
        
        // Find corresponding point on curve2
        let transformed = iso.transform_point(point, &curve1.field_order);
        println!("    maps to ({}, {}) on Curve 2", transformed.x, transformed.y);
        assert!(curve2.contains_point(&transformed));
    }
}