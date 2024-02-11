

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn str_to_num(s: &str) -> u64 {
    s.bytes()
        .fold(0_u64, |acc, b| acc * 256 + b as u64)
}

fn num_to_str(mut num: u64) -> String {
    let mut chars = Vec::new();
    while num > 0 {
        chars.insert(0, (num % 256) as u8);
        num /= 256;
    }
    String::from_utf8(chars).expect("Invalid UTF-8")
}


fn mod_inverse(e: u64, phi: u64) -> Option<u64> {
    let (mut a, mut b, mut u, mut v) = (e as i128, phi as i128, 1_i128, 0_i128);
    while a != 0 {
        let q = b / a;
        let r = b % a;
        let m = u - q * v;
        b = a;
        a = r;
        u = v;
        v = m;
    }
    if b == 1 { Some(((u + phi as i128) % phi as i128) as u64) } else { None }
}

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn main() {
    // Using larger primes that fit within u64, but still far from "large" in cryptographic terms
    let p: u64 = 5_802_923; // Large 23-bit prime
    let q: u64 = 6_700_543; // Large 23-bit prime
    let n = p * q;
    let e: u64 = 65_537; // Common choice for RSA public exponent
    let phi = (p - 1) * (q - 1);
    let d = mod_inverse(e, phi).expect("Inverse should exist");

    let message = "Hi";
    let m = str_to_num(message);

    if m >= n {
        panic!("Message too large for this key");
    }

    let encrypted = mod_pow(m, e, n);
    let decrypted = mod_pow(encrypted, d, n);
    let decrypted_message = num_to_str(decrypted);

    println!("Original: {}, Encrypted: {}, Decrypted: {}", message, encrypted, decrypted_message);
}

// Include the previously defined functions: gcd, lcm, mod_inverse, mod_pow, str_to_num, num_to_str
