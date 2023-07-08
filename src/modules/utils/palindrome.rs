use num_bigint::BigUint;


pub trait PalindromeStuff {
    /**
     * Returns OK(string, length) if the number is a palindrome.
     * Returns error otherwise.
     */
    fn is_palindrome(&self) -> bool;
}

impl PalindromeStuff for String {
    fn is_palindrome(&self) -> bool {
        let bytes = self.as_bytes();
        let len = bytes.len();

        // don't need to check if len is odd since the extra digit is in 
        // the middle
        let half = len/2;
        let mut i = 0;
        while i < half {
            if bytes[i] != bytes[len-1-i] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

impl PalindromeStuff for BigUint {
    fn is_palindrome(&self) -> bool {

        // this is about 10% faster than the version below
        let str = self.to_str_radix(10);
        let bytes = str.as_bytes();
        let len = bytes.len();
        // don't need to check if len is odd since the extra digit is in 
        // the middle
        let half = len/2;
        let mut i = 0;
        while i < half {
            if bytes[i] != bytes[len-1-i] {
                return false;
            }
            i += 1;
        }
        return true;

        // this version is 20% faster than the commented version below
        // but it is still twice as slow as u128
        /*
        let str = self.to_str_radix(10);
        let bytes = str.as_bytes();
        let mut rev = bytes.to_owned();
        rev.reverse();
        if !bytes.eq(&rev) {
            return false;
        }
        */
        /*
        let str = self.to_str_radix(10);
        // this line is 30% slower than self.to_str_radix(10);
        // let str = self.to_string();
        if str != str.chars().rev().collect::<String>() {
            return false;
        }
        */
        //return true;
    }
}

impl PalindromeStuff for u128 {
    fn is_palindrome(&self) -> bool {
        let str = self.to_string();
        if str != str.chars().rev().collect::<String>() {
            return false;
        }
        return true;
    }
}

impl PalindromeStuff for u32 {
    fn is_palindrome(&self) -> bool {
        let str = self.to_string();
        if str != str.chars().rev().collect::<String>() {
            return false;
        }
        return true;
    }
}

impl PalindromeStuff for u64 {
    fn is_palindrome(&self) -> bool {
        let str = self.to_string();
        if str != str.chars().rev().collect::<String>() {
            return false;
        }
        return true;
    }
}

