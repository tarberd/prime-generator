pub use num_bigint::BigUint;

pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Rng { state: seed }
    }

    #[allow(dead_code)]
    pub fn xorshift(&mut self) -> u32 {
        let x = self.state;

        let x = x ^ (x << 13);
        let x = x ^ (x >> 17);
        let x = x ^ (x << 5);

        self.state = x;

        x
    }

    pub fn linear_congruential(&mut self) -> u32 {
        let a = 48_271;
        let m = 2_147_483_647;
        self.state = mod_pow(a, u64::from(self.state), m) as u32;

        self.state
    }
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

pub fn miller_rabin_primality_test(candidate: BigUint) -> bool {
    let mut two_factors = 0;

    let mut rest = candidate.clone() - 1_u32;

    while rest.clone() % 2_u32 == BigUint::from(0_u32) {
        rest /= 2_u32;
        two_factors += 1;
    }

    let rounds = (candidate.bits() as f64).ln() as u32 + 1;

    for _ in 0..rounds {
        use num_bigint::RandBigInt;

        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&BigUint::from(2_u32), &(candidate.clone() - 2_u32));

        let mut x = a.modpow(&rest, &candidate);

        if x == BigUint::from(1_u32) || x == (candidate.clone() - 1_u32) {
            continue;
        }

        let mut did_break = false;

        for _ in 0..(two_factors - 1) {
            x = x.modpow(&BigUint::from(2_u32), &candidate);

            if x == (candidate.clone() - 1_u32) {
                did_break = true;
                break;
            }
        }

        if !did_break {
            return false;
        }
    }

    true
}

pub fn fermat_primality_test(candidate: BigUint) -> bool {
    if candidate == BigUint::from(2_u32) {
        return true;
    } else if candidate.clone() & BigUint::from(1_u32) != BigUint::from(1_u32) {
        return false;
    }
    BigUint::from(2_u32).modpow(&(candidate.clone() - 1_u32), &candidate) == BigUint::from(1_u32)
}

pub fn make_prime_candidate(rng: &mut Rng, bits: usize) -> BigUint {
    let mut num = BigUint::from(rng.linear_congruential());

    while num.bits() <= bits {
        num <<= 32;
        num |= BigUint::from(rng.linear_congruential());
    }

    let extra_bits = num.bits() - bits;
    num >>= extra_bits;
    num |= BigUint::from(1_u32);

    num
}
