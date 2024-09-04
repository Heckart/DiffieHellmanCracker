use rand::Rng;
#[path = "../src/eve.rs"]
mod eve;

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

    for _ in 1..100 {
        let public_base = rand::thread_rng().gen_range(2..5);
        let alice_sec = rand::thread_rng().gen_range(2..5);
        let bob_sec = rand::thread_rng().gen_range(2..5);

        let alice_broadcasts = (public_base as u64).pow(alice_sec as u32);
        let bob_broadcasts = (public_base as u64).pow(bob_sec as u32);
        let mutual_secret = (alice_broadcasts as u64).pow(bob_sec as u32);
        // which is the same as:
        // let mutual_secret = (bob_broadcasts as f64).powf(alice_sec).round();

        let results = eve::baby_eve(
            alice_broadcasts as u64,
            bob_broadcasts as u64,
            public_base as u64,
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
