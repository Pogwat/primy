pub trait UIntBits: PrimInt + Unsigned + std::ops::Sub<Output = Self> {
    fn is_power_of_two(self) -> bool;
    fn low_bit_mask(bits_to_keep: usize) -> Self;
    fn extract_low_bits(self,bits_to_keep: usize) -> Self;
    fn high_bit_mask(bits_to_keep: usize) -> Self;
    fn extract_high_bits(self,bits_to_keep: usize) -> Self;
}

// Implement the trait for ALL types that fit your unsigned math requirements
impl<I> UIntBits for I 
where
    I: PrimInt + Unsigned + std::ops::Sub<Output = Self>
{
    #[inline(always)]
    fn is_power_of_two(self) -> bool {
        // Your proven math check: BOX_SIZE > 0 && (BOX_SIZE & (BOX_SIZE - 1)) == 0
        // We use I::zero() instead of 0, and I::one() instead of 1
        self > I::zero() && (self & (self - I::one())) == I::zero()
    }

    #[inline(always)]
    fn low_bit_mask(bits_to_keep: usize) -> Self {
        if bits_to_keep == 0 {
            return I::zero();
        }
        // Your proven math trick: (1 << bits) - 1
        // We shift the generic 1 token left, then subtract the generic 1 token
        (I::one() << bits_to_keep) - I::one()
    }

    fn extract_low_bits(self,bits_to_keep: usize) -> Self {
        self & Self::low_bit_mask(bits_to_keep)
    }

    fn high_bit_mask(bits_to_keep: usize) -> Self {!Self::low_bit_mask(bits_to_keep) }
    
    fn extract_high_bits(self,bits_to_keep: usize) -> Self {
        self & Self::high_bit_mask(bits_to_keep)
    }
}