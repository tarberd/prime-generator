fn xorshift(seed: u32) -> u32 {
    let x = seed;

    let x = x ^ (x << 13);
    let x = x ^ (x >> 17);
    let x = x ^ (x << 5);
    x
}

fn main() {
    let seed = 123413241u32;

    let seed = xorshift(seed);
    println!("Random number: {:032b}", seed);
    let seed = xorshift(seed);
    println!("Random number: {:032b}", seed);
}
