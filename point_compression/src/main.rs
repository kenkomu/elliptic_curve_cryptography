extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::ops::{Add, Sub, Mul, Rem};

/// Elliptic curve parameters: y^2 = x^3 + ax + b (mod p)
#[derive(Debug, Clone)]
struct EllipticCurve {
    a: BigInt,
    b: BigInt,
    p: BigInt,
}

/// Represents a point (x, y) on the elliptic curve
#[derive(Debug, Clone)]
struct Point {
    x: BigInt,
    y: BigInt,
}

/// Computes the modular square root using the Tonelli-Shanks algorithm
fn mod_sqrt(n: &BigInt, p: &BigInt) -> Option<BigInt> {
    if n.modpow(&((p - 1u32) / 2u32), p) != BigInt::one() {
        return None; // No square root exists
    }
    Some(n.modpow(&((p + 1u32) / 4u32), p)) // Only valid for p ≡ 3 (mod 4)
}

/// Compress a point (x, y) into (x, parity_bit)
fn compress_point(point: &Point, p: &BigInt) -> (BigInt, u8) {
    let parity = if &point.y < &(*&p - &point.y) { 0 } else { 1 };
    (point.x.clone(), parity)
}

/// Decompress a point given (x, parity_bit)
fn decompress_point(x: BigInt, parity: u8, curve: &EllipticCurve) -> Option<Point> {
    let x_cubed = x.modpow(&BigInt::from(3u32), &curve.p);
    let rhs = (x_cubed + &curve.a * &x + &curve.b).rem(&curve.p); // y^2 = x^3 + ax + b mod p
    let y = mod_sqrt(&rhs, &curve.p)?;

    let y_correct = if (y.clone() < (curve.p.clone() - &y)) == (parity == 0) {
        y
    } else {
        curve.p.clone() - y
    };

    Some(Point { x, y: y_correct })
}

fn main() {
    // Define the elliptic curve: y^2 = x^3 + 2x + 3 mod 17
    let curve = EllipticCurve {
        a: BigInt::from(3),
        b: BigInt::from(4),
        p: BigInt::from(7),
    };

    // Sample point on the curve
    let point = Point {
        x: BigInt::from(2),
        y: BigInt::from(5),
    };

    // Compress
    let (compressed_x, parity) = compress_point(&point, &curve.p);
    println!("Compressed: ({}, {})", compressed_x, parity);

    // Decompress
    if let Some(decompressed_point) = decompress_point(compressed_x, parity, &curve) {
        println!(
            "Decompressed: ({}, {})",
            decompressed_point.x, decompressed_point.y
        );
    } else {
        println!("Decompression failed!");
    }
}
