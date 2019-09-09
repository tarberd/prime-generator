fn xorshift(seed: u32) -> u32 {
    let x = seed;

    let x = x ^ (x << 13);
    let x = x ^ (x >> 17);
    let x = x ^ (x << 5);
    x
}

fn linear_congruential(seed: u32) -> u32 {
    let a = 48271;
    let m = 2147483647;
    ((a * seed as u64) % m) as u32
}

fn miller_rabin_primality_test(testee: u32) -> bool {
    println!("{}", testee);
    let testee_one_off = testee - 1;

    println!("{}", testee_one_off);
    let two_factors = ((testee_one_off) as f32).log2() as u32;

    println!("{}", two_factors);
    let testee_one_off_twos_off = testee_one_off >> two_factors;

    println!("{}", testee_one_off_twos_off);

    let rounds = 10;

    for k in (0..rounds) {}

    true
}

fn main() {
    let seed = 123413241u32;

    let seed = linear_congruential(seed);
    println!("Random number: {:032b}", seed);

    let prime_test = miller_rabin_primality_test(seed);
    println!("Is prime? : {}", prime_test);
    let seed = linear_congruential(seed);
    println!("Random number: {:032b}", seed);

    let prime_test = miller_rabin_primality_test(seed);
    println!("Is prime? : {}", prime_test);
}
