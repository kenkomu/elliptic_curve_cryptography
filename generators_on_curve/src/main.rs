#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    infinity: bool,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            infinity: false,
        }
    }

    fn is_at_infinity(&self) -> bool {
        self.infinity
    }

    fn at_infinity() -> Self {
        Self {
            x: 0,
            y: 0,
            infinity: true,
        }
    }
}

fn mod_inv(a: i32, m: i32) -> i32 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, m, a.rem_euclid(m));
    while new_r != 0 {
        let quotient = r / new_r;
        t -= quotient * new_t;
        std::mem::swap(&mut t, &mut new_t);
        r -= quotient * new_r;
        std::mem::swap(&mut r, &mut new_r);
    }
    t.rem_euclid(m)
}

fn elliptic_add(p: &Point, q: &Point, a: i32, m: i32) -> Point {
    if p.is_at_infinity() {
        return q.clone();
    }
    if q.is_at_infinity() {
        return p.clone();
    }
    if p.x == q.x && (p.y != q.y || p.y == 0) {
        return Point::at_infinity();
    }

    let (x1, y1, x2, y2) = (p.x, p.y, q.x, q.y);
    let lambda = if p == q {
        ((3 * x1 * x1 + a) * mod_inv(2 * y1, m)).rem_euclid(m)
    } else {
        ((y2 - y1) * mod_inv(x2 - x1, m)).rem_euclid(m)
    };

    let x3 = (lambda * lambda - x1 - x2).rem_euclid(m);
    let y3 = (lambda * (x1 - x3) - y1).rem_euclid(m);

    Point::new(x3, y3)
}

fn scalar_mult(n: i64, p: &Point, a: i32, m: i32) -> Point {
    let mut r = Point::at_infinity();
    let mut q = p.clone();
    let mut n = n;
    
    while n > 0 {
        if n & 1 == 1 {
            r = elliptic_add(&r, &q, a, m);
        }
        q = elliptic_add(&q, &q, a, m);
        n >>= 1;
    }
    r
}

fn is_point_on_curve(x: i32, y: i32, a: i32, b: i32, m: i32) -> bool {
    let left = y.pow(2).rem_euclid(m);
    let right = (x.pow(3) + a * x + b).rem_euclid(m);
    left == right
}

fn find_all_points(a: i32, b: i32, m: i32) -> Vec<Point> {
    let mut points = Vec::new();
    
    // Add point at infinity
    points.push(Point::at_infinity());
    
    // Check all possible x and y coordinates
    for x in 0..m {
        for y in 0..m {
            if is_point_on_curve(x, y, a, b, m) {
                points.push(Point::new(x, y));
            }
        }
    }
    
    points
}

fn find_generators(points: &[Point], a: i32, m: i32) -> Vec<Point> {
    let total_points = points.len();
    let mut generators = Vec::new();
    
    for point in points {
        if point.is_at_infinity() {
            continue;
        }
        
        let mut subgroup_size = 1;
        let mut current = point.clone();
        
        while !current.is_at_infinity() && subgroup_size <= total_points {
            current = elliptic_add(&current, point, a, m);
            subgroup_size += 1;
        }
        
        if subgroup_size == total_points {
            generators.push(point.clone());
            println!("Found generator: {:?} with order {}", point, subgroup_size);
        }
    }
    
    generators
}

fn main() {
    let a = 4;
    let b = 4;
    let m = 7;
    
    println!("Finding points on curve y² = x³ + {}x + {} mod {}", a, b, m);
    
    let points = find_all_points(a, b, m);
    println!("\nFound {} points on the curve:", points.len());
    for point in &points {
        if !point.is_at_infinity() {
            println!("({}, {})", point.x, point.y);
        } else {
            println!("Point at infinity");
        }
    }
    
    println!("\nFinding generators...");
    let generators = find_generators(&points, a, m);
    println!("\nFound {} generators:", generators.len());
    for generator in &generators {
        println!("({}, {})", generator.x, generator.y);
    }
}