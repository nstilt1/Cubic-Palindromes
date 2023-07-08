use std::str::FromStr;
use std::sync::{Arc, Mutex};
use num_bigint::BigUint;
use rayon::prelude::*;

use crate::RADIX;

use crate::modules::utils::radix::{TheRadix, Reversal};
use crate::modules::utils::{
    palindrome::*,
    display::*,
    io::*,
    big_pow::*,
};

/**
 * Function to find palindromes^5
 */
pub fn find_palindromes_bigint_pow5_multithreaded_ranged(min_bits: u32, max_bits: u32) -> Vec<u128> {
    let min_u128 = (2 as u128).pow(min_bits);
    let max_u128 = (2 as u128).pow(max_bits);

    //let mut palindromes: Vec<u32> = Vec::new();
    let data: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(vec![]));
    (min_u128..=max_u128).into_par_iter().for_each(|i| {
        let n = if RADIX != 10 {
            u128::from_str(&i.to_radix(RADIX)).unwrap()
        }else{
            i
        };
        
        if n.fifth_to_str().is_palindrome() {
            let mut data = data.lock().unwrap();
            data.push(i);
        }
    });
    
    //println!("Num cubed palindromes: {:?}, cubed ones:\n{}", palindromes.len(), palindromes.print_cubes());
    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    let first_output = format!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<u128> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    let second_output = format!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    
    let bit_analysis = v.print_bit_counts();

    let write_result = write_file(min_bits, max_bits, &format!("{}{}{}", &first_output, &second_output, &bit_analysis));
    if write_result.is_err() {
        println!("Error writing file: {:?}", write_result.unwrap_err());
    }

    return v;
}

/**
 * Possibly the most efficient test for checking all numbers of non-base 10
 */
pub fn find_palindromes_base_n(min_bits: u32, max_bits: u32) {
    let min: u64 = min_bits.into();
    let max: u64 = (2 as u64).pow(max_bits);
    let radix = RADIX as u32;
    let data: Arc<Mutex<Vec<BigUint>>> = Arc::new(Mutex::new(vec![]));
    (min..=max).into_par_iter().map_init(|| (), |_, i| {
        BigUint::from_radix_be(BigUint::from(i).to_radix_be(radix).as_slice(), 10).unwrap()
    }).for_each(|big| {
        if big.cube_to_str().is_palindrome() {
            let mut data = data.lock().unwrap();
            data.push(big);
        }
    });

    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    let first_output = format!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<BigUint> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    let second_output = format!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    
    let bit_analysis = v.print_bit_counts();

    let write_result = write_file(min_bits, max_bits, &format!("{}{}{}", &first_output, &second_output, &bit_analysis));
    if write_result.is_err() {
        println!("Error writing file: {:?}", write_result.unwrap_err());
    }
}

/**
 * Possibly the most efficient way to check base 10 numbers
 */
pub fn find_palindromes_base_10(min_bits: u32, max_bits: u32) {
    let min: u64 = min_bits.into();
    let max: u64 = (2 as u64).pow(max_bits);
    let data: Arc<Mutex<Vec<BigUint>>> = Arc::new(Mutex::new(vec![]));
    (min..=max).into_par_iter().map_init(|| (), |_, i| {
        BigUint::from(i)
    }).for_each(|big| {
        if big.cube_to_str().is_palindrome() {
            let mut data = data.lock().unwrap();
            data.push(big);
        }
    });

    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    let first_output = format!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<BigUint> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    let second_output = format!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    
    let bit_analysis = v.print_bit_counts();

    let write_result = write_file(min_bits, max_bits, &format!("{}{}{}", &first_output, &second_output, &bit_analysis));
    if write_result.is_err() {
        println!("Error writing file: {:?}", write_result.unwrap_err());
    }
}

/**
 * Not quite the most efficient way to check for cubic palindromes, but 
 * if you want to try to find cubic palindromes with more than 4 1s in it,
 * then this is what you would use. But it does allow us to check much bigger numbers faster.
 * It would be more efficient if it only allowed a maximum of 4 1s in the number.
 */
pub fn find_palindromes_exclusively(min_bits: u32, max_bits: u32) {
    let min: u64 = min_bits.into();
    let max: u64 = (2 as u64).pow(max_bits);
    let data: Arc<Mutex<Vec<BigUint>>> = Arc::new(Mutex::new(vec![]));
    (min..=max).into_par_iter().map_init(|| (), |_, i| {
        BigUint::from_radix_be(BigUint::from(i).to_radix_be(2).as_slice(), 10).unwrap()
    }).for_each(|big| {
        let str = big.to_str_radix(10);
        let pal_even = BigUint::from_str(
            &format!("{}{}", &str, &str.reverse())
        ).unwrap();
        let pal_odd_0 = BigUint::from_str(
            &format!("{}0{}", &str, &str.reverse())
        ).unwrap();
        let pal_odd_1 = BigUint::from_str(
            &format!("{}1{}", &str, &str.reverse())
        ).unwrap();

        let pal_even_is_pali = pal_even.cube_to_str().is_palindrome();
        let pal_odd_0_is_pali = pal_odd_0.cube_to_str().is_palindrome();
        let pal_odd_1_is_pali = pal_odd_1.cube_to_str().is_palindrome();
        
        if pal_even_is_pali || pal_odd_0_is_pali || pal_odd_1_is_pali {
            let mut data = data.lock().unwrap();
            if pal_even_is_pali {
                data.push(pal_even);
            }
            if pal_odd_0_is_pali {
                data.push(pal_odd_0);
            }
            if pal_odd_1_is_pali {
                data.push(pal_odd_1);
            }
        }
    });

    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    let first_output = format!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<BigUint> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    let second_output = format!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    
    let bit_analysis = v.print_bit_counts();

    let write_result = write_file_exclusive(min_bits, max_bits, &format!("{}{}{}", &first_output, &second_output, &bit_analysis));
    if write_result.is_err() {
        println!("Error writing file: {:?}", write_result.unwrap_err());
    }
}

pub fn find_palindromes_bigint_pow3_multithreaded_ranged(min_bits: u32, max_bits: u32) -> Vec<u128> {
    let min_u128 = (2 as u128).pow(min_bits);
    let max_u128 = (2 as u128).pow(max_bits);

    //let mut palindromes: Vec<u32> = Vec::new();
    let data: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(vec![]));
    (min_u128..=max_u128).into_par_iter().for_each(|i| {
        let n = if RADIX != 10 {
            u128::from_str(&i.to_radix(RADIX)).unwrap()
        }else{
            i
        };
        if i.cube_to_str().is_palindrome() {
            let mut data = data.lock().unwrap();
            data.push(i);
        }
    });
    
    //println!("Num cubed palindromes: {:?}, cubed ones:\n{}", palindromes.len(), palindromes.print_cubes());
    let d = data.lock().unwrap();
    let mut v = d.to_vec();
    v.sort();
    let len = v.len();
    let first_output = format!("Num cubed palindromes: {:?}, cubed ones:\n{}", len, v.print_cubes(10));

    let mut non_palindromes: Vec<u128> = Vec::new();
    for n in v.to_owned() {
        //let radixed = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(10);
        if !n.is_palindrome() {
            non_palindromes.push(n);
        }
    }
    let second_output = format!("\nNon palindrome cubic palindromes: \n{}", non_palindromes.print_cubes(10));
    
    let bit_analysis = v.print_bit_counts();

    let write_result = write_file(min_bits, max_bits, &format!("{}{}{}", &first_output, &second_output, &bit_analysis));
    if write_result.is_err() {
        println!("Error writing file: {:?}", write_result.unwrap_err());
    }
    
    return v;
}