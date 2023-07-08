use num_bigint::BigUint;
use std::{str::FromStr, ops::AddAssign};
use crate::modules::utils::{
    display::*,
    palindrome::*,
    big_pow::*
};
use crate::RADIX;

pub fn find_palindromes_bigint_single_threaded(max_bits: u32) {
    let mut num: BigUint = BigUint::from_str("0").unwrap();
    let max = BigUint::from_str("2").unwrap().pow(max_bits);
    let one = BigUint::from_str("1").unwrap();

    let mut palindromes: Vec<BigUint> = Vec::new();

    while num < max {
        let cubed = num.cubed();
        if cubed < num {
            break;
        }
        if cubed.is_palindrome() {
            palindromes.push(num.clone());
        }
        num.add_assign(one.to_owned());
    }
    println!("Num cubed palindromes: {:?}, cubed ones:\n{}", palindromes.len(), palindromes.print_cubes(RADIX));

}

fn find_palindromes_u128_single_thread(max_bits: u32) {
    println!("Testing palindromes");

    let mut num_u = 0 as u128;
    let max = (2 as u128).pow(max_bits);
    
    let mut palindromes_u128: Vec<u128> = Vec::new();
    let mut palindromes_cubed_u128: Vec<u128> = Vec::new();

    while num_u < max {

        /*
        if num_u.is_palindrome() {
            palindromes_u128.push(num_u);
            if num_u.cubed().is_palindrome() {
                palindromes_cubed_u128.push(num_u);
            }
        }
        */
        let cubed = num_u.cubed();
        if cubed < BigUint::from_bytes_le(&num_u.to_le_bytes()) {
            break;
        }
        if cubed.is_palindrome() {
            palindromes_cubed_u128.push(num_u.to_owned());
        }
        num_u.add_assign(1);
    }
    println!("Num palindromes: {:?}, num cubed palindromes: {:?}, cubed ones:\n{}", palindromes_u128.len(), palindromes_cubed_u128.len(), palindromes_cubed_u128.print_cubes(RADIX));
}