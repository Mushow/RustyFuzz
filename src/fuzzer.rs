use rand::Rng;
use crate::c_interface::call_vulnerable_function;

pub struct Fuzzer {
    max_input_length: usize,
}

impl Fuzzer {
    pub fn new(max_input_length: usize) -> Self {
        Self { max_input_length }
    }

    pub fn generate_random_input(&self) -> String {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(1..=self.max_input_length);
        (0..length)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    }

    pub fn fuzz(&self) {
        let input = self.generate_random_input();
        println!("Testing with input: {}", input);
        call_vulnerable_function(&input);
    }
}
