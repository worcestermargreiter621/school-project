// Generate a random string of length 10 using Rust's rand crate
use rand::{thread_rng, Rng};
let mut rng = thread_rng();
let random_string: String = (0..10)
    .map(|_| {
        let index = rng.gen_range(0..26);
        let char = ('a' as u8 + index) as char;
        char
    })
    .collect();
println!("Random string: {}", random_string);
