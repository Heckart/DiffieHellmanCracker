pub fn baby_eve(alice_broadcasts: u64, bob_broadcasts: u64, public_base: u64) -> [u64; 3] {
    // Purpose:     Crack baby DH
    // Parameters:  alice's broadcast, bob's broadcast, and the public base
    // User-input:  None
    // Prints:      Nothing
    // Returns:     Should return an array of 3 unsigned ints:
    //              Alice's secret, Bob's secret, shared secret
    // Modifies:    Nothing
    // Calls:       ?
    // Tests:       unit_test_babydh.rs
    // Status:      Done, correct, but is it ideal?

    let mut ret: [u64; 3] = [0, 0, 0];

    let a_broadcast_f64: f64 = alice_broadcasts as f64;
    let b_broadcast_f64: f64 = bob_broadcasts as f64;
    let pub_base_f64: f64 = public_base as f64;

    let a_secret = a_broadcast_f64.log(pub_base_f64);
    let a_secret_u64: u64 = a_secret as u64;

    let b_secret = b_broadcast_f64.log(pub_base_f64);
    let b_secret_u64: u64 = b_secret as u64;

    let mut_sec: u64 = alice_broadcasts.pow(b_secret_u64 as u32);

    ret[0] = a_secret_u64;
    ret[1] = b_secret_u64;
    ret[2] = mut_sec;

    return ret;
}

pub fn big_eve(
    alice_broadcasts: u64,
    bob_broadcasts: u64,
    public_base: u64,
    public_modulus: u64,
) -> [u64; 3] {
    // Purpose:      Crack real DH (albeit not with huge numbers)
    // Parameters:   Alice's broadcast, Bob's broadcast, the public base and modulus of DH.
    // User-input:   None
    // Prints:       Nothing
    // Returns:      Should return an array of 3 ints:
    //               Alice's secret, Bob's secret, shared secret
    // Modifies:     Nothing
    // Calls:        ?
    // Test:         ./unit_tests/unit_test_babydh.rs
    // Status:       Done

    // must be brute forced

    let mut ret: [u64; 3] = [0, 0, 0];

    let mut guess: u32 = 0;

    let mut a_priv: u64 = 0;

    let mut b_priv: u64 = 0;

    // find alice's secret
    loop {
        guess += 1;

        let init_guess: u64 = public_base.pow(guess);
        let fin_guess: u64 = init_guess.rem_euclid(public_modulus);

        if fin_guess == alice_broadcasts {
            a_priv = guess as u64;
            break;
        }
    }

    guess = 0;
    loop {
        guess += 1;

        let init_guess: u64 = public_base.pow(guess);
        let fin_guess: u64 = init_guess.rem_euclid(public_modulus);

        if fin_guess == bob_broadcasts {
            b_priv = guess as u64;
            break;
        }
    }

    let shared_sec_p1: u64 = bob_broadcasts.pow(a_priv as u32);
    let shared_sec: u64 = shared_sec_p1.rem_euclid(public_modulus);

    ret[0] = a_priv;
    ret[1] = b_priv;
    ret[2] = shared_sec;

    return ret;
}
