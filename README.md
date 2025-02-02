This educational rust program is capable of cracking the Diffie-Hellman key exchange. There are two functions to crack two different versions of DH: `baby_eve` and `big_eve`

# Baby Diffie-Hellman
The `baby_eve` function in the `eve.rs` file is capable of quickly and successfully cracking a weaker conceptual cousin to DH, using secret exponents without mod.
This weaker version is described here:

1. Alice and Bob pick a shared base:
$`SharedBase`$

2. Alice sends Bob the shared base to the power of her secret:
$`AliceBroadcasts = SharedBase^{AliceSec}`$

3. Bob sends Alice the shared base to the power of his secret:
$`BobBroadcasts = SharedBase^{BobSec}`$

4. Alice generates the shared secret:
$`BobBroadcasts^{AliceSec}`$

5. Bob generates same shared secret:
$`AliceBroadcasts^{BobSec}`$

# Real Diffie-Hellman
This version mimics the baby version in structure, but with a real brute force from the bottom up.

# Running this Code
All code is in the `src/` directory. The main driver is in `src/main.rs`. Below are some commands that you may find to be helpful

- `cargo build` : Compiles the code and puts it in `target/debug/name_specified_in_cargo.toml`
- `cargo run` : Compiles and runs the code, allowing interaction with it for testing.
    - May require a --bin if you have multiple binaries (as this project does with it's unit test).
- `cargo run arg1 arg2` : Compiles and runs your code, passing along any number of arguments you need.
- `cargo run --bin=randomized_test_big 123` for example, making sure your exit code is 123 (or the same as any number you specify).

# Background Reading Material
To gain a better understanding of what exactly is happening here, I suggest reading from the following sources:

https://cs-mst.gitlab.io/index/Classes/Security/Content/CryptoMath.html

https://cs-mst.gitlab.io/index/Classes/Security/Content/AsymmetricEncryption.html#diffie-hellman

https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange

https://en.wikipedia.org/wiki/Primitive_root_modulo_n

# PS
The number types in Rust only go up to 64-bit (128-bit if you are using u128). This is not large enough to allow for large keys to be cracked. All inputs will be small enough to be handled in the u64 variable types. The usage of a BigInt is not required for the basic baby eve.
