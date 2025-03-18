fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..101);
    println!("The number is {}", number);
}
