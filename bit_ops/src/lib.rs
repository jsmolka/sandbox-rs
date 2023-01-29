use std::ops::Range;

pub trait BitOps {
    fn mask(size: usize) -> Self;
    fn is_set(&self, index: usize) -> bool;
    fn bit(&self, index: usize) -> Self;
    fn set_bit(&mut self, index: usize, value: Self);
    fn bits(&self, range: Range<usize>) -> Self;
    fn set_bits(&mut self, range: Range<usize>, value: Self);
}

impl BitOps for u32 {
    #[inline(always)]
    fn mask(size: usize) -> Self {
        assert!(size <= Self::BITS as usize);
        Self::MAX >> (Self::BITS as usize - size)
    }

    #[inline(always)]
    fn is_set(&self, index: usize) -> bool {
        assert!(index < Self::BITS as usize);
        self & (1 << index) != 0
    }

    #[inline(always)]
    fn bit(&self, index: usize) -> Self {
        assert!(index < Self::BITS as usize);
        (self >> index) & 0x1
    }

    #[inline(always)]
    fn set_bit(&mut self, index: usize, value: Self) {
        assert!(index < Self::BITS as usize);
        *self = (*self & !(1 << index)) | ((value & 0x1) << index);
    }

    #[inline(always)]
    fn bits(&self, range: Range<usize>) -> Self {
        assert!(range.start < range.end);
        assert!(range.end <= Self::BITS as usize);
        let mask = Self::mask(range.len());
        (self >> range.start) & mask
    }

    #[inline(always)]
    fn set_bits(&mut self, range: Range<usize>, value: Self) {
        assert!(range.start < range.end);
        assert!(range.end <= Self::BITS as usize);
        let mask = Self::mask(range.len());
        *self = (*self & !(mask << range.start)) | ((value & mask) << range.start);
    }
}
