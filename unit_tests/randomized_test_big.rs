use rand::Rng;
#[path = "../src/eve.rs"]
mod eve;

fn is_prime(number: u64) -> bool {
    for potential_factor in 2..=((number as f64).sqrt() as u64) {
        if number % potential_factor == 0 {
            return false;
        }
    }
    true
}

fn prim_roots(number: u64) -> Vec<u64> {
    let mut roots: Vec<u64> = Vec::new();
    let mut o = 1;
    let mut r = 2;
    while r < number {
        let mut k = r.pow(o).rem_euclid(number);
        while k > 1 {
            o = o + 1;
            k = (k * r) % number;
        }
        if o as u64 == (number - 1) {
            roots.push(r);
        }
        o = 1;
        r = r + 1;
    }
    return roots;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Must include returncode as arg")
    }
    let desired_retcode = &args[1];
    let desired_retcode: i32 = desired_retcode
        .trim()
        .parse()
        .expect("Please type a number!");

    for _ in 0..63 {
        let mut public_modulus;
        loop {
            // prime mod of 3 and generator of 2 don't always work...
            public_modulus = rand::thread_rng().gen_range(5..=9);
            if is_prime(public_modulus) {
                break;
            }
        }
        let roots = prim_roots(public_modulus);
        let public_base = roots[rand::thread_rng().gen_range(0..roots.len())];
        let alice_sec = rand::thread_rng().gen_range(2..5);
        let bob_sec = rand::thread_rng().gen_range(2..5);

        let alice_broadcasts = public_base.pow(alice_sec as u32).rem_euclid(public_modulus);
        let bob_broadcasts = public_base.pow(bob_sec as u32).rem_euclid(public_modulus);
        let mutual_secret = alice_broadcasts
            .pow(bob_sec as u32)
            .rem_euclid(public_modulus);
        // let mutual_secret = bob_broadcasts.pow(alice_sec as u32).rem_euclid(public_modulus);
        // println!("alice_broadcasts {alice_broadcasts}");
        // println!("bob_broadcasts {bob_broadcasts}");
        // println!("public_base {public_base}");
        // println!("public_modulus {public_modulus}");

        let results = eve::big_eve(
            alice_broadcasts,
            bob_broadcasts,
            public_base,
            public_modulus,
        );
        let alice_cracked = results[0];
        let bob_cracked = results[1];
        let secret_cracked = results[2];

        if (alice_sec != alice_cracked)
            || (bob_sec != bob_cracked)
            || (mutual_secret as u64 != secret_cracked)
        {
            std::process::exit(1);
        }
    }
    std::process::exit(desired_retcode);
}
