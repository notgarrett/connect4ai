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
        let mut bitmap = Bitmap::<SIZE>::new();
        let mut mask = Bitmap::<SIZE>::new();

        for x in 0..WIDTH as usize {
            bitmap.set(x, true);
            mask.set(x, true);
        }

        Self { bitmap, mask }
    }

    pub fn get(&self, pos: usize) -> bool {
        self.bitmap.get(pos)
    }

    pub fn get_mask(&self, pos: usize) -> bool {
        self.mask.get(pos)
    }

    pub fn set_top(&mut self, pos: usize) -> Result<BitResult, BitError> {
        if pos > SIZE {
            return Err(BitError::OutOfBounds(pos));
        }

        self.bitmap.set(pos, true);
        Ok(BitResult::Success)
    }

    pub fn set(&mut self, pos: usize, val: bool) -> Result<BitResult, BitError> {
        if pos > SIZE - WIDTH as usize {
            return Err(BitError::OutOfBounds(pos));
        }

        self.bitmap.set(pos, val);
        self.mask.set(pos, true);

        Ok(BitResult::Success)
    }

    pub fn get_bottom(&self, col: usize) -> Option<usize> {
        if self.bitmap.get(col + SIZE - WIDTH as usize) == true {
            return None;
        }

        for x in (0..HEIGHT as usize).rev() {
            if self.bitmap.get(col + (x * 7)) {
                return Some(col + (x * 7));
            }
        }

        None
    }

    // This is for player 2
    pub fn xor(&self) -> Bitmap<SIZE> {
        self.bitmap ^ self.mask
    }

    pub fn and(&self) -> Bitmap<SIZE> {
        self.bitmap & self.mask
    }
}
