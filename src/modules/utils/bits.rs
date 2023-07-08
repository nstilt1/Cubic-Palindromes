

pub trait Bits {
    fn bits(&self) -> usize;
}

impl Bits for u64 {
    fn bits(&self) -> usize {
        if self == &0 {
            return 0 as usize;
        }
        return (self.ilog2() + 1) as usize;
    }
}

impl Bits for u128 {
    fn bits(&self) -> usize {
        if self == &0 {
            return 0 as usize;
        }
        return (self.ilog2() + 1) as usize;
    }
}