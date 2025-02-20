const P: i64 = 17;


//convert a short Weierstrass curve to a Montgomery curve
//y^2 = x^3 + ax + b
//to
//By^2 = x^3 + Ax^2 + x

fn short_weierstrass_to_montgomery(a: i64, b: i64) -> Option<(i64, i64)> {
    // Step 1 : Find z0, a root of polynomial z = x^3 + ax + b mod p
    let mut z0 = 0;
    for x in 0..P {
        let z = (x.pow(3) + a*x + b) % P;
        if z == 0 {
            z0 = x;
            break;
        }
    }
    if z0 == 0 {
        return None;
    }
    //step2: calculate s = (sqrt(3*z0^2 + a )) ^(-1) mod p
    let term = (3*z0.pow(2) + a) % P;
    if let Some(sqrt_term) = modular_sqrt(term, P) {
        let s = mod_inverse(sqrt_term, P)?;
        //step 3: calculate A = 3* z0 * s mod p
        let A = (3*z0*s) % P;

        //B is equal to s in the formulation
        let b_montgomery = s;
        return Some((A, b_montgomery));
    } else {
        return None;
    }

}
//HELPER function to compute the modular square root
fn modular_sqrt(a: i64, p: i64) -> Option<i64> {
    let mut result = 0;
    for x in 0..p {
        if (x.pow(2) - a) % p == 0 {
            result = x;
            break;
        }
    }
    if result == 0 {
        return None;
    }
    return Some(result);
}

//helper function to compute the modular square root
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

//helper function to compute the modular inverse
fn mod_add(a: i64, b: i64, m: i64) -> i64 {
    let sum = (a + b) % m;
    if sum < 0 {
        sum + m
    } else {
        sum
    }
}

fn main()   {
    let a = 8;
    let b = 2;
    let (a_montgomery, b_montgomery) = short_weierstrass_to_montgomery(a, b).unwrap();
    println!("A = {}, B = {}", a_montgomery, b_montgomery);
}