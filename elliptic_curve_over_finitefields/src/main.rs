#[derive(Debug, Clone, Copy)]
struct Point {
    x: Option<i64>,
    y: Option<i64>,
}

struct EllipticCurve {
    a: i64,
    b: i64,
    p: i64,
}

impl EllipticCurve {
    fn add_points(&self, p1: &Point, p2: &Point) -> Option<Point> {
        if p1.x.is_none() && p1.y.is_none() {
            return Some(*p2);
        }
        if p2.x.is_none() && p2.y.is_none() {
            return Some(*p1);
        }
        if p1.x == p2.x && p1.y == p2.y {
            return self.double(*p1);
        }
        if p1.x == p2.x && p1.y == Some(mod_neg(-p2.y.unwrap(), self.p)) {
            return Some(Point { x: None, y: None });
        }

        let lambda = if p1.x == p2.x {
            // Double point
            (3 * p1.x.unwrap().pow(2) + self.a) * self.mod_inv(2 * p1.y.unwrap(), self.p).unwrap()
        } else {
            // Point addition
            (p2.y.unwrap() - p1.y.unwrap()) * self.mod_inv(p2.x.unwrap() - p1.x.unwrap(), self.p).unwrap()
        };
        let lambda = mod_neg(lambda, self.p);

        let x3 = mod_neg(lambda.pow(2) - p1.x.unwrap() - p2.x.unwrap(), self.p);
        let y3 = mod_neg(lambda * (p1.x.unwrap() - x3) - p1.y.unwrap(), self.p);
        Some(Point { x: Some(x3), y: Some(y3) })
    }

    fn double(&self, p: Point) -> Option<Point> {
        self.add_points(&p, &p)
    }

    fn is_on_curve(&self, p: &Point) -> bool {
        p.y.map_or(false, |y| (y.pow(2) - (p.x.unwrap().pow(3) + self.a * p.x.unwrap() + self.b)) % self.p == 0)
    }

    fn mod_inv(&self, a: i64, m: i64) -> Option<i64> {
        let mut mn = (m, a);
        let mut xy = (0, 1);
        while mn.1 != 0 {
            xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
            mn = (mn.1, mn.0 % mn.1);
        }
        if mn.0 > 1 {
            None
        } else {
            Some(mod_neg(xy.0, m))
        }
    }
}

fn mod_neg(x: i64, m: i64) -> i64 {
    let x = x % m;
    if x < 0 {
        x + m
    } else {
        x
    }
}

fn main() {
    let curve = EllipticCurve {
        a: 4,
        b: 713,
        p: 713,
    };

    let p1 = Point { x: Some(488), y: Some(488) };
    let p2 = Point { x: Some(522), y: Some(502) };

    let result_add = curve.add_points(&p1, &p2);
    println!("Point.Additions: {:?}", result_add);

    let result_double = curve.double(p1);
    println!("Point.Double: {:?}", result_double);

    println!("p1_on_curve: {}", curve.is_on_curve(&p1));
    println!("p2_on_curve: {}", curve.is_on_curve(&p2));
}