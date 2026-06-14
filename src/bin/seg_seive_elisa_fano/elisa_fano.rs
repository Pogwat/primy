//SUppose we have eaquly size boxes that hold sequential integers
// i.e [0,1,2]  [3,4,5]  [6,7,8]
//To store only specfic numbers in these boxe slal we need is their idnex and box number
// (BoxNumber+Index)*BoxSize = Number, SInce BoxSize Is constant:3, we only need to store indexs and BoxNUmbers to get our numbers
// To Achive these we can store number of numbers we weeant to store per box and the index in these boxes of those numbers
// Say we wanted to store 2,7 in these 3 boxes
//We can store what box they are in by storing how many special numbeers are in each box seperated by a 0 wheen no integers are in that box
//1,0,1
//We then store the indexs opf these special numbers in their boxes
//3 (as a 3 bit val),2 (as a 3 bit val)
//This is the elisa-fano encoding

//We need a bit array to do this, but rust dosent have u1 and bools take 1 bytes
use crate::smallest_uint;
use num_traits::PrimInt;
use num_traits::Unsigned;

struct BitVec<const BOX_SIZE_IN_BITS:u8, SmallestUIntThatCanHoldABox> 
where
    SmallestUIntThatCanHoldABox: PrimInt + Unsigned + std::ops::AddAssign,
{
    bytes: Vec<SmallestUIntThatCanHoldABox>
}

impl <const BOX_SIZE_IN_BITS:u8, SmallestUIntThatCanHoldABox>BitVec<BOX_SIZE_IN_BITS,SmallestUIntThatCanHoldABox> 
where
    SmallestUIntThatCanHoldABox: PrimInt + Unsigned + std::ops::AddAssign,
{
    pub const TypeBits:u64 = (size_of::<SmallestUIntThatCanHoldABox>()*8) as u64;

    //2^n = 2^(n-1) + 2^(n-2)... + 2^(n-n) +1 so 2^n -1 = 2^(n-1) + 2^(n-2)... + 2^(n-n) = 11111.... in binary, this is a bitmask for all numbers below 2^n
    // (bitmask for all numbers below 2^n) & 2^n == 0. i.e. 1000 & 0111 ==0
    pub const IS_BOX_SIZE_POWER_OF_2:() = if !((BOX_SIZE_IN_BITS & (BOX_SIZE_IN_BITS - 1)) == 0) {
        panic!("COMPILE ERROR: BOX_SIZE_IN_BITS must be a power of 2");
    }; 
    
}