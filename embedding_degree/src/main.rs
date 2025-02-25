use num_bigint::{BigUint};
use num_traits::{Zero, One};


fn compute_embedding_degree(p: BigUint, r: BigUint) -> Option<u32> {
    let mut k = 1u32;
    let mut power = BigUint::one();

    //Iterate untill we find the smallest k such that r divides (p^k - 1)
    loop {
        power = power * &p;
        let power = power.clone() - BigUint::one();
        if power % &r == BigUint::zero() {
            return Some(k);
        }
        k += 1;

        if k > 100000 {
            return None;
        }
    }
}

fn main() {
    let p = BigUint::from(13u32);
    let r = BigUint::from(2u32);

    let embedding_degree = compute_embedding_degree(p, r).unwrap();
    println!("Embedding degree: {}", embedding_degree);
}