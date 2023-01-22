use std::ops::Range;

pub trait BitOps {
    fn mask(size: usize) -> Self;
    fn is_set(&self, index: usize) -> bool;
    fn bit(&self, index: usize) -> Self;
    fn set_bit(&mut self, index: usize, value: Self);
    fn bits(&self, range: Range<usize>) -> Self;
    fn set_bits(&mut self, range: Range<usize>, value: Self);
}

impl BitOps for u8 {
    #[inline(always)]
    fn mask(size: usize) -> Self {
        debug_assert!(size <= Self::BITS as usize);
        Self::MAX >> (Self::BITS as usize - size)
    }

    #[inline(always)]
    fn is_set(&self, index: usize) -> bool {
        debug_assert!(index < Self::BITS as usize);
        self & (1 << index) != 0
    }

    #[inline(always)]
    fn bit(&self, index: usize) -> Self {
        debug_assert!(index < Self::BITS as usize);
        (self >> index) & 0x1
    }

    #[inline(always)]
    fn set_bit(&mut self, index: usize, value: Self) {
        debug_assert!(index < Self::BITS as usize);
        *self = (*self & !(1 << index)) | ((value & 0x1) << index);
    }

    #[inline(always)]
    fn bits(&self, range: Range<usize>) -> Self {
        debug_assert!(range.start < range.end);
        debug_assert!(range.end <= Self::BITS as usize);
        let mask = Self::mask(range.end - range.start);
        (self >> range.start) & mask
    }

    #[inline(always)]
    fn set_bits(&mut self, range: Range<usize>, value: Self) {
        debug_assert!(range.start < range.end);
        debug_assert!(range.end <= Self::BITS as usize);
        let mask = Self::mask(range.end - range.start);
        *self = (*self & !(mask << range.start)) | ((value & mask) << range.start);
    }
}

fn main() {
    let mut v = 0x00_u8;
    v.set_bits(0..8, 0xFF);
    assert_eq!(v, 0xFF);
    v.set_bits(2..6, 0x00);
    assert_eq!(v, 0xC3);
}
