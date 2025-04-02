use std::fmt;

//A struct to represent elements
#[derive(Debug, Clone, PartialEq, Copy)]
struct F5x2 {
    a: u8, //coefficient of 1
    b: u8, //coefficient of t
}

impl fmt::Display for F5x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),
            (0, b) => write!(f, "{}t", b),
            (a, b) => write!(f, "{} + {}t", a, b),
            
        }
    }
    
}

impl F5x2 {
    //create a new field element
    fn new(a: u8, b: u8) -> Self {
        F5x2 {
            a: a % 5,
            b: b % 5,
        }
    }

    //add two field elements
    fn add(self, other: F5x2) -> F5x2 {
    F5x2 {  
        a: (self.a +other.a) % 5,
        b: (self.b + other.b) % 5 
    }
}
     
     //multiply two elements of mathbb F{5^2}
        fn mul(self, other: F5x2) -> F5x2 {
            let a = self.a as i16;
            let b = self.b as i16;
            let c = other.a as i16;
            let d = other.b as i16;


            //Polynomial multiplication
            let ac = (a * c) % 5;
            let bd = (b * d) % 5;
            let ad_bc = (a * d + b * c) % 5;

            //reduction modulo t^2 + 2
            let new_a = (ac + 3 * bd) % 5;
            let new_b = ad_bc % 5;

            F5x2::new(new_a as u8, new_b as u8)
        }

        //subtract two elements of mathbb F{5^2}
        fn sub(self, other: F5x2) -> F5x2 {
            F5x2 {
                a: (self.a + 5 - other.a) % 5,
                b: (self.b + 5 - other.b) % 5,
            }
        }

        //divide two elements of mathbb F{5^2}
        fn div(self, other: F5x2) -> F5x2 {
            //find the inverse of the other element
            let inv = other.inverse();
            //multiply by the inverse
            self.mul(inv)
        }

        //find the inverse of an element of mathbb F{5^2}
        fn inverse(self) -> F5x2 {
            let a = self.a as i16;
            let b = self.b as i16;

            let denominator =  (a * a - 3 * b * b).rem_euclid(5) as u8;
            let inv_denominator = Self::mod_inverse(denominator, 5);

            let new_a = (a * inv_denominator as i16).rem_euclid(5) as u8;
            let new_b = (5 - (b * inv_denominator as i16).rem_euclid(5)) as u8;

            F5x2::new(new_a, new_b)
        }

        //find the modular inverse of a number
        fn mod_inverse(x: u8, p: u8) -> u8 {
            for i in 1..p {
                if (x as u16 * i as u16) % p as u16 == 1 {
                    return i;
                }
            }
            1
        }
    }

//a struct to represent a point on the elliptic curve
#[derive(Copy, Clone, Debug)]

//use option with F5x2 
struct Point {
    x: Option<F5x2>,
    y: Option<F5x2>,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_infinity() {
            write!(f, "Point at infinity")
        } else {
            write!(f, "({}, {})", self.x.unwrap(), self.y.unwrap())
        }
    }
}

impl Point {
    //create a new point
    fn new(x: Option<F5x2>, y: Option<F5x2>) -> Self {
        Point { x, y }
    }

    //check if point is at infinity
    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

fn point_add(p: Point, q: Point, a: F5x2) -> Point {
    //check if either point is infinity
    if p.is_infinity() {
        return q;
    }
    if q.is_infinity() {
        return p;
    }

    let (x1, y1) = (p.x.unwrap(), p.y.unwrap());
    let (x2, y2) = (q.x.unwrap(), q.y.unwrap());

    if x1 == x2 && y1 != y2 {
        return Point::new(None, None);
    }

    let lambda = if x1 == x2 && y1 == y2 {
        // Point doubling
        let numerator = x1.mul(x1).mul(F5x2::new(3, 0)).add(a);
        let denominator = y1.mul(F5x2::new(2, 0));
        numerator.div(denominator)
    } else {
        // Point addition
        let numerator = y2.sub(y1);
        let denominator = x2.sub(x1);
        numerator.div(denominator)
    };

    let x3 = lambda.mul(lambda).sub(x1).sub(x2);
    let y3 = lambda.mul(x1.sub(x3)).sub(y1);

    Point::new(Some(x3), Some(y3))
}


fn main() {
    // Define the elliptic curve parameters
    let a = F5x2::new(1, 0);
    let _b = F5x2::new(1, 0);  // Prefixed with underscore to indicate intentional non-use

    // Define two points on the curve
    let p = Point::new(Some(F5x2::new(3, 4)), Some(F5x2::new(4, 3)));
    let q = Point::new(Some(F5x2::new(3, 3)), Some(F5x2::new(3, 0)));

    // Add the points
    let r = point_add(p, q, a);
    
    println!("point p : {}", p);
    println!("point q : {}", q);

    println!("Result: {}", r);
}