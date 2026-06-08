use std::num::NonZeroUsize;
use std::iter::Peekable;
pub struct SegSeive<const SEG_SIZE: usize> {
    pub seive: [bool;SEG_SIZE],
    pub current_prime_idx:Option<usize>,
    pub num_of_loops:usize,
    pub range:usize
}

impl <const SEG_SIZE:usize>SegSeive<SEG_SIZE> {
    pub fn new(range:usize) -> Self{
        Self {
            seive:[true;SEG_SIZE],
            current_prime_idx:None,
            num_of_loops:0,
            range
        }
    }

    pub const START_NUM:usize =3;
    pub const STEP:usize =2;

    pub fn clear_seive(&mut self) {self.seive.fill(true);}

    pub fn bump_seive(&mut self) {
        self.num_of_loops+=1;
        self.current_prime_idx = None;
        self.clear_seive()
    }
    
    pub fn guess_dex(&self, local_index:usize) -> usize {
        (self.seg_start_globdex()+local_index)*Self::STEP+Self::START_NUM
    }

    pub fn index(&self, local_index:usize) -> Option<NonZeroUsize> {
        return if self.seive[local_index]==true {
            NonZeroUsize::new(self.guess_dex(local_index))
        } else {None}
    }

    pub fn find_set(&self, start:usize) -> Option<usize> {
        self.seive[start..].iter().position(|num| *num).map(|relative_idx| start + relative_idx)
    }

    pub fn global_value_to_global_idx(global_value:usize) -> usize {
        (global_value-Self::START_NUM)/Self::STEP
    }



    pub fn seg_start(&self) -> usize {  guess_dex(0) }
    pub fn seg_end(&self) -> usize {  self.seg_start() + Self::STEP*SEG_SIZE }
    pub fn is_value_in_seg(&self, value:usize) -> bool {
        value<=self.seg_end() && value>= self.seg_start()
    }

    pub fn seg_start_globdex(&self) -> usize { self.num_of_loops*SEG_SIZE}
    pub fn seg_end_globdex(&self) -> usize { self.seg_start_globdex()+SEG_SIZE-1}

    pub fn global_idx_to_local_idx(&self, global_idx:usize) -> Option<usize> {
        
    }

    pub fn next_global_multiples_idx(global_idx:usize, multiple:usize) -> usize {
        let first_multiple_idx=Self::global_value_to_global_idx(multiple);
        let global_idx=global_idx.max(first_multiple_idx);
        let idx_difference = global_idx - first_multiple_idx;
        first_multiple_idx +idx_difference.div_ceil(multiple) * multiple
    }

}