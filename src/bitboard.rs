extern crate bitmaps;
use crate::game::{HEIGHT, WIDTH};
use bitmaps::Bitmap;

const SIZE: usize = (WIDTH * HEIGHT + WIDTH) as usize;

#[derive(Debug)]
pub enum BitError {
    OutOfBounds(usize),
}

pub enum BitResult {
    Success,
}

#[derive(Copy, Clone)]
pub struct Bitboard {
    bitmap: Bitmap<SIZE>,
    mask: Bitmap<SIZE>,
}

impl Bitboard {
    pub fn new() -> Self {
        Self {
            bitmap: Bitmap::<SIZE>::new(),
            mask: Bitmap::<SIZE>::new(),
        }
    }

    pub fn get(&self, pos: usize) -> bool {
        self.bitmap.get(pos)
    }

    pub fn set(&mut self, pos: usize, val: bool) -> Result<BitResult, BitError> {
        if self.bitmap.set(pos, val) {
            Ok(BitResult::Success)
        } else {
            Err(BitError::OutOfBounds(pos))
        }
    }
}
