pub mod mapping {

pub trait SmallestUInt {type Type;}
pub struct UIntRouter<const N:usize>;

impl SmallestUInt for UIntRouter<255>  { type Type = u8;  }
impl SmallestUInt for UIntRouter<65535> { type Type = u16; }
impl SmallestUInt for UIntRouter<4294967295> { type Type = u32; }
impl SmallestUInt for UIntRouter<18446744073709551615> { type Type = u64; }

pub const fn find_smallest_type_num(val: usize) -> usize {
    if val <= 255 { 255} 
    else if val <= 65535 { 65535} 
    else if val <= 4294967295 { 4294967295 } 
    else if val <= 18446744073709551615 { 18446744073709551615}
    else {0}
}
#[macro_export]
macro_rules! smallest_uint {
    ($val:expr) => {<UIntRouter<{find_smallest_type_num($val)}> as SmallestUInt>::Type}
}

}