use std::fmt;

/// Represents a point on an elliptic curve
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Creates a new point
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    /// Returns the point at infinity
    fn infinity() -> Self {
        Point {
            x: -1,
            y: -1,
        }
    }

    /// Checks if this point is the point at infinity
    fn is_infinity(&self) -> bool {
        self.x == -1 && self.y == -1
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_infinity() {
            write!(f, "Point at infinity")
        } else {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

/// Represents an elliptic curve y² = x³ + ax + b (mod p)
struct EllipticCurve {
    a: i64,
    b: i64,
    p: i64,
}

impl EllipticCurve {
    /// Creates a new elliptic curve with the given parameters
    fn new(a: i64, b: i64, p: i64) -> Result<Self, &'static str> {
        if p <= 0 {
            return Err("Modulus must be positive");
        }
        Ok(EllipticCurve { a, b, p })
    }

    /// Verifies if a point lies on the curve
    fn contains(&self, point: &Point) -> bool {
        if point.is_infinity() {
            return true;
        }

        // Ensure all operations are done modulo p and keep values positive
        let x3 = point.x.pow(3) % self.p;
        let ax = (self.a * point.x) % self.p;
        let right = (x3 + ax + self.b) % self.p;
        let left = (point.y * point.y) % self.p;
        
        // Ensure positive modulo
        let left = (left + self.p) % self.p;
        let right = (right + self.p) % self.p;
        
        left == right
    }

    /// Adds two points on the curve
    fn add(&self, p1: &Point, p2: &Point) -> Result<Point, &'static str> {
        if p1.is_infinity() {
            return Ok(*p2);
        }
        if p2.is_infinity() {
            return Ok(*p1);
        }

        // Normalize coordinates to be positive
        let x1 = (p1.x % self.p + self.p) % self.p;
        let y1 = (p1.y % self.p + self.p) % self.p;
        let x2 = (p2.x % self.p + self.p) % self.p;
        let y2 = (p2.y % self.p + self.p) % self.p;

        if x1 == x2 && y1 != y2 {
            return Ok(Point::infinity());
        }

        let lambda = if p1 == p2 {
            // Point doubling
            if y1 == 0 {
                return Ok(Point::infinity());
            }
            
            let numerator = (3 * x1 * x1 + self.a) % self.p;
            let denominator = (2 * y1) % self.p;
            
            match mod_inverse(denominator, self.p) {
                Some(inv) => (numerator * inv) % self.p,
                None => return Err("Could not compute modular inverse"),
            }
        } else {
            // Point addition
            let numerator = (y2 - y1 + self.p) % self.p;
            let denominator = (x2 - x1 + self.p) % self.p;
            
            match mod_inverse(denominator, self.p) {
                Some(inv) => (numerator * inv) % self.p,
                None => return Err("Could not compute modular inverse"),
            }
        };

        let x3 = (lambda * lambda - x1 - x2) % self.p;
        let y3 = (lambda * (x1 - x3) - y1) % self.p;

        // Ensure positive modulo
        let x3 = (x3 + self.p) % self.p;
        let y3 = (y3 + self.p) % self.p;

        Ok(Point::new(x3, y3))
    }

    /// Finds the order of a point on the curve
    fn find_order(&self, point: &Point) -> Result<i64, &'static str> {
        if !self.contains(point) {
            return Err("Point is not on the curve");
        }

        println!("Starting point: {}", point);
        println!("Verifying point is on curve: {}", self.contains(point));

        let mut current = *point;
        let mut order = 1;

        while !current.is_infinity() {
            let next = self.add(&current, point)?;
            println!("Step {}: Adding {} to {} gives {}", 
                    order, point, current, next);
            
            current = next;
            order += 1;

            if order > self.p + 1 {
                return Err("Order computation exceeded curve bounds");
            }
        }

        Ok(order)
    }
}

/// Computes the modular multiplicative inverse
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (mut t, mut newt) = (0, 1);
    let (mut r, mut newr) = (m, a);

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }

    if r > 1 {
        return None;
    }

    if t < 0 {
        t += m;
    }

    Some(t)
}

fn main() -> Result<(), &'static str> {
    let curve = EllipticCurve::new(8, 2, 17)?;
    let point = Point::new(0, 5);

    // Verify the point is on the curve
    if !curve.contains(&point) {
        println!("Point verification failed!");
        println!("Left side (y²): {}", (point.y * point.y) % 7);
        println!("Right side (x³ + ax + b): {}", (point.x.pow(3) + 4 * point.x + 4) % 7);
        return Err("Initial point is not on the curve");
    }

    println!("Point {} is on the curve y² = x³ + {}x + {} mod {}", 
             point, curve.a, curve.b, curve.p);

    match curve.find_order(&point) {
        Ok(order) => println!("Order of {} is: {}", point, order),
        Err(e) => println!("Error computing order: {}", e),
    }

    Ok(())
}