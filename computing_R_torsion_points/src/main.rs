use std::fmt;

#[derive(Clone, Copy)]
struct Point{
    x: i64,
    y: i64,
    infinity: bool
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.infinity {
            write!(f, "Point {{ x: 0, y: 0 }}")
        } else {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }
}

impl Point {
    fn is_at_infinity(&self) -> bool {
        self.infinity
    }
    fn at_inifinity() -> Self {
        Point { x: 0, y: 0, infinity: true }
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, m, ((a % m) + m) % m);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = t;
        t = new_t;
        new_t = tmp - q * new_t;
        let tmp = r;
        r = new_r;
        new_r = tmp - q * new_r;
    }
    if r > 1 {
        panic!("{} is not invertible mod {}", a, m);
    }
    ((t % m) + m) % m
}

fn elliptic_add(p: &Point, q: &Point, a: i64, m: i64) -> Point {
    if p.is_at_infinity() {
        return *q;
    }
    if q.is_at_infinity() {
        return *p;
    }
    if p.x == q.x && ((p.y + q.y) % m == 0) {
        return Point::at_inifinity();
    }
    let s = if p.x == q.x {
        ((3 * p.x % m * p.x % m + a) % m * mod_inv(2 * p.y % m, m)) % m
    } else {
        ((q.y - p.y + m) % m * mod_inv((q.x - p.x + m) % m, m)) % m
    };
    let x = ((s * s - p.x - q.x) % m + m) % m;
    let y = ((s * (p.x - x) - p.y) % m + m) % m;
    Point { x, y, infinity: false }
}

fn scalar_mult(n: i64, p: &Point, a: i64, m: i64) -> Point {
    let mut r = Point::at_inifinity();
    let mut q = *p;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            r = elliptic_add(&r, &q, a, m);
        }
        q = elliptic_add(&q, &q, a, m);
        n /= 2;
    }
    r
}

fn is_on_curve(p: &Point, a: i64, b: i64, m: i64) -> bool {
    if p.is_at_infinity() {
        return true;
    }
    let x = p.x;
    let y = p.y;
    (y * y - x * x * x - a * x - b) % m == 0
}

fn find_r_torsion_points(r: i64, a: i64, b: i64, m: i64) -> Vec<Point> {
    let mut torsion_points = vec![Point::at_inifinity()];
    for x in 0..m {
        for y in 0..m {
            let p = Point { x, y, infinity: false };
            if is_on_curve(&p, a, b, m) {
                let mut q = p;
                for i in 1..r {
                    q = elliptic_add(&q, &p, a, m);
                    if q.is_at_infinity() {
                        break;
                    }
                }
                if q.is_at_infinity() {
                    torsion_points.push(p);
                }
            }
        }
    }
    torsion_points
}



fn main() {
    let a = 8;
    let b = 8;
    let m = 13;
    let r = 5;
    let torsion_points = find_r_torsion_points(r, a, b, m);
    for p in torsion_points {
        println!("{:?}", p);
    }
}