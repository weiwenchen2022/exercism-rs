pub fn private_key(p: u64) -> u64 {
    // todo!("Pick a private key greater than 1 and less than {p}")
    use rand::Rng;

    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // todo!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // todo!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    mod_pow(b_pub, a, p)
}

fn mod_pow(mut b: u64, mut e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut r = 1_u64;
    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            r = r.wrapping_mul(b) % m;
        }

        b = b.wrapping_mul(b) % m;
        e >>= 1;
    }

    r
}
