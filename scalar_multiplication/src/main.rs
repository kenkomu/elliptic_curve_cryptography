#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    infinity: bool, // Flag indicating if the point is at infinity
}

impl Point {
    fn is_at_infinity(&self) -> bool {
        self.infinity
    }

    fn at_infinity() -> Point {
        Point {
            x: 0,
            y: 0,
            infinity: true,
        }
    }
}

fn mod_pos(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, m, a % m);
    while new_r != 0 {
        let quotient = r / new_r;
        let temp = t;
        t = new_t;
        new_t = temp - quotient * new_t;
        let temp = r;
        r = new_r;
        new_r = temp - quotient * new_r;
    }
    if r > 1 {
        panic!("{} is not invertible mod {}", a, m);
    }
    if t < 0 {
        t += m;
    }
    t
}

fn add(p1: &Point, p2: &Point, a: i64, m: i64) -> Point {
    if p1.is_at_infinity() {
        return p2.clone();
    }
    if p2.is_at_infinity() {
        return p1.clone();
    }

    if p1.x == p2.x && p1.y == p2.y {
        // Point doubling
        if p1.y == 0 {
            return Point::at_infinity();
        }
        let lambda = mod_pos(
            (3 * p1.x * p1.x + a) * mod_inv(2 * p1.y, m),
            m
        );
        let x3 = mod_pos(lambda * lambda - 2 * p1.x, m);
        let y3 = mod_pos(lambda * (p1.x - x3) - p1.y, m);
        Point {
            x: x3,
            y: y3,
            infinity: false,
        }
    } else {
        // Point addition
        if p1.x == p2.x {
            return Point::at_infinity();
        }
        let lambda = mod_pos(
            (p2.y - p1.y) * mod_inv(p2.x - p1.x, m),
            m
        );
        let x3 = mod_pos(lambda * lambda - p1.x - p2.x, m);
        let y3 = mod_pos(lambda * (p1.x - x3) - p1.y, m);
        Point {
            x: x3,
            y: y3,
            infinity: false,
        }
    }
}
fn scalar_mult(p: &Point, k: i64, a: i64, m: i64) -> Point {
    let mut result = Point::at_infinity();
    let mut addend = p.clone();
    let mut k = k;
    while k > 0 {
        if k & 1 == 1 {
            result = add(&result, &addend, a, m);
        }
        addend = add(&addend, &addend, a, m);
        k >>= 1;
    }
    result
}
fn main() {
    let a = 4;
    let m = 19;
    let k = 7;
    let p1 = Point {
        x: 6,
        y: 4,
        infinity: false,
    };

let np = scalar_mult(&p1, k, a, m);
   if np.is_at_infinity() {
       println!("The result is at infinity");
   } else {
       println!("The result is {:?}", np);
   }
}