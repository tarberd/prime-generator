struct Rng {
    state: u32,
}

impl Rng {
    fn new(seed: u32) -> Self {
        Rng { state: seed }
    }

    fn xorshift(&mut self) -> u32 {
        let x = self.state;

        let x = x ^ (x << 13);
        let x = x ^ (x >> 17);
        let x = x ^ (x << 5);

        self.state = x;

        x
    }

    fn linear_congruential(&mut self) -> u32 {
        let a = 48271;
        let m = 2147483647;
        self.state = mod_pow(a, self.state as u64, m) as u32;

        self.state
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn miller_rabin_primality_test(testee: u32) -> bool {
    let mut two_factors = 0;

    let mut rest = testee - 1;
    while rest % 2 == 0 {
        rest /= 2;
        two_factors += 1;
    }

    let rounds = 12;
    for _ in 0..rounds {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(2, testee - 2);

        let mut x = mod_pow(a as u64, rest as u64, testee as u64) as u32;

        if x == 1 || x == (testee - 1) {
            continue;
        }

        let mut did_break = false;

        for _ in 0..(two_factors - 1) {
            x = mod_pow(x as u64, 2 as u64, testee as u64) as u32;

            if x == (testee - 1) {
                did_break = true;
                break;
            }
        }

        if did_break == false {
            return false;
        }
    }

    true
}

fn main() {
    let mut rng = Rng::new(123413242);

    let seed = rng.linear_congruential() | 1;
    println!("Random number: {}", seed);

    let mut prime_test = miller_rabin_primality_test(seed);
    println!("Is prime? : {}", prime_test);

    while prime_test == false {
        let seed = rng.linear_congruential() | 1;
        println!("Random number: {}", seed);

        prime_test = miller_rabin_primality_test(seed);
        println!("Is prime? : {}", prime_test);
    }
}
