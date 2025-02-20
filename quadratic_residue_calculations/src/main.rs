//compute all quadratic residues modulo p using euler's criterion
fn list_quadratic_residues(p: u64) -> Result<Vec<u64>, &'static str> {
    if p < 2 || p % 2 == 0 {
        return Err("p must be an odd prime");
    }
    let mut residues = Vec::new();
    for x in 1..p {
        if is_coprime(x, p) && is_quadratic_residue(x, p)? {
            residues.push(x);
        }
    }

    Ok(residues)
}

// Dtermine if a number is coprime to p
fn is_coprime(x: u64, p: u64) -> bool {
    gcd(x, p) == 1
}

//check if x is a quadratic residue modulo p using the euler's criterion
fn is_quadratic_residue(x: u64, p: u64) -> Result<bool, &'static str> {
    let exponent = (p - 1) / 2;
    let result = mod_exp(x, exponent, p);

    if result == 1 {
        Ok(true)
    } else if result == p - 1 {
        Ok(false)
    } else {
        Err("x is not a quadratic residue modulo p")
    }
}

//compute GCD
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// compute modular exponentiation
fn mod_exp(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exponent /= 2;
    }

    result
}

fn main() {
    let p = 13;
    match list_quadratic_residues(p) {
        Ok(residues) => {
            println!("Quadratic residues modulo {} are: {:?}", p, residues);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}