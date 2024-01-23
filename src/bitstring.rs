use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXor, BitXorAssign, Shl, Shr};

use crate::game::{HEIGHT, WIDTH};

const SIZE: usize = (WIDTH * HEIGHT + WIDTH) as usize;

#[derive(Debug)]
pub enum BitError {
    OutOfBounds(usize),
}

pub enum BitResult {
    Success,
}

#[derive(Copy, Clone)]
pub struct BitString {
    bits: u64,
}

impl BitString {
    pub fn new() -> Self {
        Self { bits: 0_u64 }
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }
    //
    pub fn top_mask(col: usize) -> u64 {
        ((1 as u64) << (HEIGHT - 1)) << col * (HEIGHT as usize + 1)
    }

    pub fn bottom_mask(col: usize) -> u64 {
        (1 as u64) << col * (HEIGHT as usize + 1)
    }

    pub fn column_mask(col: usize) -> u64 {
        (((1 as u64) << HEIGHT) - 1) << col * (HEIGHT as usize + 1)
    }
}

impl BitAnd for BitString {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitAnd<u64> for BitString {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Self {
            bits: self.bits & rhs,
        }
    }
}

impl BitXor for BitString {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitXor<u64> for BitString {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        Self {
            bits: self.bits ^ rhs,
        }
    }
}

impl Shl for BitString {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits << rhs.bits,
        }
    }
}

impl Shl<u64> for BitString {
    type Output = Self;
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            bits: self.bits << rhs,
        }
    }
}

impl Shr for BitString {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits >> rhs.bits,
        }
    }
}

impl Shr<u64> for BitString {
    type Output = Self;
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            bits: self.bits >> rhs,
        }
    }
}

impl BitOrAssign for BitString {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl BitOrAssign<u64> for BitString {
    fn bitor_assign(&mut self, rhs: u64) {
        self.bits |= rhs
    }
}

impl BitAndAssign for BitString {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits
    }
}

impl BitAndAssign<u64> for BitString {
    fn bitand_assign(&mut self, rhs: u64) {
        self.bits &= rhs
    }
}

impl BitXorAssign for BitString {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits
    }
}

impl BitXorAssign<u64> for BitString {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.bits ^= rhs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_xor() {}
}
