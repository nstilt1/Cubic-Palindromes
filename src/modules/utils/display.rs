use std::str::FromStr;

use num_bigint::BigUint;

use super::{big_pow::*, bits::Bits};


pub trait PalinVec {
    fn print_cubes(&self, radix: u8) -> String;
    fn print_bit_counts(&self) -> String {
        return "".to_owned();
    }
}

impl PalinVec for Vec<u128> {
    fn print_cubes(&self, radix: u8) -> String {
        let mut output = "".to_owned();
        let mut joined = "".to_owned();
        for n in self {
            let base_r = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(radix as u32);
            let base_10 = u128::from_str(&base_r).unwrap();
            output.push_str(&format!("{} ^3 = {}\n", &base_10, base_10.to_owned().cube_to_str()));
            if joined.len() > 0 {
                joined.push(',');
            }
            joined.push_str(&n.to_string());
        }
        output.push('\n');
        output.push_str(&joined);
        return output;
    }
    fn print_bit_counts(&self) -> String {
        let mut output = "\nBit counts\n".to_owned();
        for n in self {
            let cube = n.cubed();
            output.push_str(&format!("{} bits for {}\n{} bits for its cube, {}\n", n.bits(), n, cube.bits(), &cube.to_string()));
        }
        return output;
    }
}

impl PalinVec for Vec<BigUint> {
    fn print_cubes(&self, radix: u8) -> String {
        let mut output = "".to_owned();
        let mut joined = "".to_owned();
        for n in self {
            let radixed = n.to_str_radix(radix as u32);
            output.push_str(&format!("{} ^3 = {}\n", radixed, n.to_owned().cube_to_str()));
            if joined.len() > 0 {
                joined.push(',');
            }
            joined.push_str(&n.to_string());
        }
        output.push('\n');
        output.push_str(&joined);
        return output;
    }
}

impl PalinVec for Vec<u32> {
    fn print_cubes(&self, radix: u8) -> String {
        let mut output = "".to_owned();
        let mut joined = "".to_owned();
        for n in self {
            let radixed_ = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(radix as u32);
            output.push_str(&format!("{} ^3 = {}\n", radixed_, n.to_owned().cube_to_str()));
            if joined.len() > 0 {
                joined.push(',');
            }
            joined.push_str(&n.to_string());
        }
        output.push('\n');
        output.push_str(&joined);
        return output;
    }
}

impl PalinVec for Vec<u64> {
    fn print_cubes(&self, radix: u8) -> String {
        let mut output = "".to_owned();
        let mut joined = "".to_owned();
        for n in self {
            let base_r = BigUint::from_bytes_le(&n.to_le_bytes()).to_str_radix(radix as u32);
            let base_10 = u64::from_str(&base_r).unwrap();
            output.push_str(&format!("{} ^3 = {}\n", &base_10, base_10.to_owned().cube_to_str()));
            if joined.len() > 0 {
                joined.push(',');
            }
            joined.push_str(&n.to_string());
        }
        output.push('\n');
        output.push_str(&joined);
        return output;
    }
    fn print_bit_counts(&self) -> String {
        let mut output = "".to_owned();
        for n in self {
            let cube = n.cubed();
            output.push_str(&format!("{} bits for {}\n{} bits for its cube, {}", n.bits(), n, cube.bits(), &cube.to_string()));
        }
        return output;
    }
}