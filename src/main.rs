mod modules;
use modules::{
    tests::{
        single_thread::*,
        threaded_basic::*,
        threaded_ranged::*,
    },
    utils::{
        time::*,
    }
};

// change this to change the base of the numbers you want to check
pub const RADIX: u8 = 2;

fn main() {

    let start_time = time_ms();

    // these could take a long time if your CPU has a low core count
    //find_palindromes_test(0, 32);
    //find_palindromes_exclusively(26,30);

    // these should run okay on most systems
    //find_palindromes_base_n(0, 20);
    find_palindromes_base_10(0, 26);
    //find_palindromes_exclusively(0, 15);
    let end_time = time_ms();
    let diff1 = end_time - start_time;
    println!("Test 1 took {} ms", diff1);
}