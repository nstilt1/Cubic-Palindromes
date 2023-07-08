use num_integer::div_rem;

pub trait Reversal {
    fn reverse(&self) -> String;
}

impl Reversal for String {
    fn reverse(&self) -> String {
        return self.chars().rev().collect::<String>();
    }
}

pub trait TheRadix {
    fn to_radix(&self, radix: u8) -> String;
}

impl TheRadix for u128 {
    fn to_radix(&self, radix: u8) -> String {
        let mut x = self.to_owned();
        let mut result = "".to_owned();
        loop {
            let div_rem = div_rem(x, radix as u128);
            x = div_rem.0;
            result.push_str(&div_rem.1.to_string());
            if x == 0 {
                break;
            }
        }
        // reverse string
        return result.reverse();
        //return BigUint::from_str(&result.reverse()).unwrap();
        //return BigUint::from_u32(0).unwrap();
    }
}