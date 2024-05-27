mod c_interface;
mod fuzzer;

use fuzzer::Fuzzer;

fn main() {
    let fuzzer = Fuzzer::new(60);

    for _ in 0..1000 {
        fuzzer.fuzz();
    }
}
