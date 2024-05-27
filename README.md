# RustyFuzz

RustyFuzz is a simple fuzzer implemented in Rust to test a C function for vulnerabilities. It generates random inputs and feeds them to the target function to detect potential crashes or memory corruption issues.

## Getting Started

These instructions will guide you through setting up and running RustyFuzz on Linux, macOS, and Windows.

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can download it from [rustup.rs](https://rustup.rs).
- **C Compiler**: A C compiler like `gcc` or `clang` is required to compile the shared library.

### Setting Up

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rustyfuzz.git
    cd rustyfuzz
    ```

2. Create the shared library.

#### On Linux:

   ```sh
   gcc -shared -o libvulnerable.so vulnerable.c
   ```

#### On macOS:

   ```sh
   gcc -shared -o libvulnerable.dylib vulnerable.c
   ```

#### On Windows

   ```sh
   gcc -shared -o libvulnerable.dll vulnerable.c "-Wl,--out-implib=libvulnerable.lib"
   ```

### Build and Run the Rust Project:
```sh
cargo build
cargo run
```

### Usage:
- Fuzzing: The fuzz method in src/fuzzer.rs generates random inputs and calls the vulnerable C function. If a crash is detected, it logs the input that caused the crash.


- Modifying Input Generation: Customize the input generation logic in the generate_random_input method to include different types of characters or patterns.

### Example Output

```sh
╭─ ~/Desktop/RustyFuzz > main                                                                                                            TRAP ✘ 
╰─ cargo run
Testing with input: "kKVKrDX2NAIxZoTqYQKcBSEF88aZbElnWPRjpbwyCPpRcLn"
Testing with input: "bog79yIi3hb8SB9i8m1U3SsZ9gRs022qV4"
Testing with input: "jqFzMnbUC82wd0N3NU3qKFEQLqI"
Testing with input: "hTtVD3K36TgjAA0JwAmgLPSGA2VPNfVkkDA2bNzlm60M"
Testing with input: "y5qw5WVyoAyVJqJMpLhkYRyft6eQyC13p"
Testing with input: "TWzgUeK7Tg9S"
Testing with input: "Ce8LhcN8s1uGnwoUhD7rEDsraIa7VbN1w4C4igzl8V6kPdZJPIU"
[1]    37946 trace trap  cargo run
```