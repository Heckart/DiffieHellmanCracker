use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io;

use eve::big_eve;
mod eve;

fn main() {
    // Purpose:    Driver for DH.
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      None
    // Status:     Done

    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let filename = &args[1];
            let content = read_to_string(filename).unwrap();
            let parts: Vec<&str> = content.split_whitespace().collect();

            let alice_broadcast_str: String = String::from(parts[0]);
            let alice_broadcast: u64 = alice_broadcast_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let bob_broadcast_str: String = String::from(parts[1]);
            let bob_broadcast: u64 = bob_broadcast_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let public_base_str: String = String::from(parts[2]);
            let public_base: u64 = public_base_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let public_modulus_str: String = String::from(parts[3]);
            let public_modulus: u64 = public_modulus_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let results = big_eve(alice_broadcast, bob_broadcast, public_base, public_modulus);

            let new_file_name = &args[2];
            let data = results
                .iter()
                .map(|&num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            if let Err(e) = fs::write(new_file_name, data) {
                eprintln!("Fail! {}", e);
            }
        }

        _ => {
            let mut alice_broadcast_str: String = String::new();
            io::stdin()
                .read_line(&mut alice_broadcast_str)
                .expect("Failed to read line");
            let alice_broadcast: u64 = alice_broadcast_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let mut bob_broadcast_str: String = String::new();
            io::stdin()
                .read_line(&mut bob_broadcast_str)
                .expect("Failed to read line");
            let bob_broadcast: u64 = bob_broadcast_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let mut public_base_str: String = String::new();
            io::stdin()
                .read_line(&mut public_base_str)
                .expect("Failed to read line");
            let public_base: u64 = public_base_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let mut public_modulus_str: String = String::new();
            io::stdin()
                .read_line(&mut public_modulus_str)
                .expect("Failed to read line");
            let public_modulus: u64 = public_modulus_str
                .trim()
                .parse()
                .expect("Input not an integer");

            let results = big_eve(alice_broadcast, bob_broadcast, public_base, public_modulus);
            println!("{} {} {}", results[0], results[1], results[2]);
        }
    }
}
