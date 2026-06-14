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
use bitvec::vec;
use bitvec::prelude::BitVec;

struct ElisaFano {
    num_of_elements_per_box: BitVec,
    indexes: BitVec
}

impl ElisaFano{
    fn bit_box_size(max_num:u64,num_of_elements:u64) -> u32 {
        (max_num/num_of_elements).ilog2()
    } 

    fn num_of_boxes(&self) -> usize {
        self.num_of_elements_per_box.count_zeros() //0's seperate boxes
    }

    // fn nth_box_start(&self, box_num: u64) -> usize

    // fn push(&mut self,num:u64) {

    // }
    
}