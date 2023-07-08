use std::str::FromStr;

use num_bigint::BigUint;



pub trait BigPow {
    fn cubed(&self) -> BigUint;
    fn fifth(&self) -> BigUint;
    fn cube_to_str(&self) -> String {
        return self.cubed().to_str_radix(10);
    }
    fn fifth_to_str(&self) -> String {
        return self.fifth().to_str_radix(10);
    }
}

impl BigPow for BigUint {
    fn cubed(&self) -> BigUint {
        return self * self * self;
    }
    fn fifth(&self) -> BigUint {
        let squared = self * self;
        return &squared * &squared * self;
    }
}

impl BigPow for String {
    fn cubed(&self) -> BigUint {
        return BigUint::from_str(&self).unwrap().cubed();
    }
    fn fifth(&self) -> BigUint {
        return BigUint::from_str(&self).unwrap().fifth();
    }
}

impl BigPow for u128 {
    fn cubed(&self) -> BigUint {
        return BigUint::from(*self).cubed();
        //return BigUint::from_bytes_le(&self.to_le_bytes()).cubed();
    }
    fn fifth(&self) -> BigUint {
        return BigUint::from(*self).fifth();
        //return BigUint::from_bytes_be(&self.to_le_bytes()).fifth();
    }
}

impl BigPow for u64 {
    fn cubed(&self) -> BigUint {
        return BigUint::from(*self).cubed();
        //return BigUint::from_bytes_le(&self.to_le_bytes()).cubed();
    }
    fn fifth(&self) -> BigUint {
        return BigUint::from(*self).fifth();
        //return BigUint::from_bytes_be(&self.to_le_bytes()).fifth();
    }
}

impl BigPow for u32 {
    fn cubed(&self) -> BigUint {
        return BigUint::from(*self).cubed();
        //return BigUint::from_bytes_le(&self.to_le_bytes()).cubed();
    }
    fn fifth(&self) -> BigUint {
        return BigUint::from(*self).fifth();
        //return BigUint::from_bytes_be(&self.to_le_bytes()).fifth();
    }
}