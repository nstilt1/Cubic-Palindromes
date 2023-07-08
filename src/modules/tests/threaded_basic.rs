use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use radix_fmt::radix;

use crate::RADIX;
use crate::modules::utils::{
    palindrome::*,
    display::*,
    io::*,
    big_pow::*
};

pub fn find_palindromes_bigint_multithreaded(max_bits: u32) -> Vec<u128> {
    let zero = BigUint::from_str("0").unwrap();
    let max_u32 = (2 as u128).pow(max_bits);
    let max = BigUint::from_str("2").unwrap().pow(max_bits);
    let one = BigUint::from_str("1").unwrap();

    //let mut palindromes: Vec<u32> = Vec::new();
    let data: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(vec![]));
    (0..=max_u32).into_par_iter().for_each(|i| {
        if RADIX != 10 {
            let base_n = radix(i, RADIX).to_string();
            let grown = u128::from_str(&base_n).unwrap();
            if grown.cube_to_str().is_palindrome() {
                let mut data = data.lock().unwrap();
                data.push(grown);
            }
        }else{
            if i.cube_to_str().is_palindrome() {
                let mut data = data.lock().unwrap();
                data.push(i);
            }
        }
        let base_n = radix(i, RADIX).to_string();
        let grown = u128::from_str(&base_n).unwrap();
        if grown.cube_to_str().is_palindrome() {
            let mut data = data.lock().unwrap();
            data.push(grown);
        }
    });
    
    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    println!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<u128> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    println!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    return v;
}