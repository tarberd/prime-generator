use prime_generator::*;

fn main() {
    let mut rng = Rng::new(0xDEAD_BEAF);

    let prime_size = 40;

    let mut candidate = BigUint::from(0_u32);

    let mut prime_test = false;
    while !prime_test {
        candidate = make_prime_candidate(&mut rng, prime_size);

        prime_test = miller_rabin_primality_test(candidate.clone());

        // prime_test = fermat_primality_test(candidate);
    }

    println!("Random number: {}", candidate);
    println!("Is prime? : {}", prime_test);
}
