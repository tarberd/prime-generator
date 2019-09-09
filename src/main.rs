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
    (a * seed) % m
}

fn main() {
    let seed = 123413241u32;

    let seed = linear_congruential(seed);
    println!("Random number: {:032b}", seed);
}
