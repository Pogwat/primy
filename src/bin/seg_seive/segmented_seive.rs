use std::iter::Peekable;
use crate::array_collect::ArrayCollectExt;
use std::num::NonZeroUsize;

pub struct SegmentedSeive<const SEG_SIZE:  usize > {
    pub segmented_seive : [Option<NonZeroUsize>;SEG_SIZE],
    pub last_primes_idx:Option<usize>,
    pub range:usize,
    pub num_of_loops:usize
}

impl <const SEG_SIZE:  usize> SegmentedSeive<SEG_SIZE> {

    pub const FIRST_START_NUM:usize=3;
    pub const SEG_SIZE:usize = SEG_SIZE;
    pub const STEP:usize = 2;
    pub const GLOB_IDX_OFFSET_FROM_NORMAL:usize = (Self::FIRST_START_NUM-1)/Self::STEP;
    pub const NUMS_PER_SEG:usize = Self::SEG_SIZE*Self::STEP;

    pub fn new(range:usize) -> Self {
        Self {
            segmented_seive: Self::new_seive(Self::FIRST_START_NUM, range),
            last_primes_idx:None,
            range,
            num_of_loops:0
        }
    }
    pub fn guess_dex(&self,index:usize)-> usize {self.num_of_loops*Self::NUMS_PER_SEG+Self::FIRST_START_NUM+index*Self::STEP}
    pub fn seg_start(&self) -> usize {self.guess_dex(0)}
    pub fn seg_end(&self) -> usize {self.guess_dex(Self::NUMS_PER_SEG-1)}
    pub fn is_value_within_seg(&self, value:usize) -> bool{ value <= self.seg_end() && value >= self.seg_start() }

    pub fn seg_start_globdex(&self) -> usize {self.num_of_loops*SEG_SIZE}
    pub fn seg_end_globdex(&self) -> usize {self.seg_start_globdex()+SEG_SIZE-1}
    pub fn is_global_idx_within_seg(&self, global_idx:usize) -> bool {
        global_idx>= self.seg_start_globdex() && global_idx<= self.seg_end_globdex()
    }

    pub fn global_idx_to_local_idx(&self, global_idx:usize) -> Option<usize>{
        return if self.is_global_idx_within_seg(global_idx) {
            Some(global_idx-self.seg_start_globdex())
        } else {None}
    }

    pub fn local_value_to_local_idx(&self,local_value:usize) -> usize{ (local_value-self.seg_start())/Self::STEP }
    pub fn local_idx_to_global_idx(&self, local_idx:usize) -> usize {self.seg_start_globdex()+local_idx}

    pub fn global_value_to_global_idx(global_value:usize) -> usize {(global_value-Self::FIRST_START_NUM)/Self::STEP}
    pub fn global_value_to_local_idx(&self, global_value:usize) -> Option<usize> {
        return if self.is_value_within_seg(global_value) {
            Some(self.local_value_to_local_idx(global_value))
        } else {None}
    }

    pub fn next_global_multiples_idx(global_idx:usize, multiple:usize) -> usize {
        let first_multiple_idx=Self::global_value_to_global_idx(multiple);
        if global_idx <= first_multiple_idx { first_multiple_idx }
        else {
            let idx_difference = global_idx - first_multiple_idx;
            let offset_to_lower_multiple = idx_difference % multiple;
            global_idx - offset_to_lower_multiple + multiple
        }
    }

    //8195!!!! is not prime pls fix me!

    pub fn next_local_multiples_idx(&self,local_idx:usize, multiple:usize) -> Option<usize> {
        self.global_idx_to_local_idx(Self::next_global_multiples_idx(self.local_idx_to_global_idx(local_idx), multiple))
    }

    pub fn next_local_multiples_iter(&self, local_start_idx:usize,multiple:usize) -> Option<Peekable<impl Iterator<Item = & Option<NonZeroUsize>>>> {
        return if let Some(first_local_multiple_idx) = self.next_local_multiples_idx(local_start_idx,multiple) {
            Some(self.segmented_seive[first_local_multiple_idx..].iter().step_by(multiple).peekable())
        } else {None}
    } 

    pub fn mut_next_local_multiples_iter(&mut self , local_start_idx:usize,multiple:usize) -> Option<Peekable<impl Iterator<Item = &mut Option<NonZeroUsize>>>> {
        return if let Some(first_local_multiple_idx) = self.next_local_multiples_idx(local_start_idx,multiple) {
            Some(self.segmented_seive[first_local_multiple_idx..].iter_mut().step_by(multiple).peekable())
        } else {None}
    } 

    pub fn remove_multiples_in_iter<'a>(&mut self, multiples_iter:impl Iterator<Item = &'a usize>) {
        multiples_iter.for_each(|multiple| self.remove_multiples(*multiple))
    }

    pub fn remove_multiples(&mut self, multiple:usize) {
        if let Some(multiples_iter) = self.mut_next_local_multiples_iter(self.last_primes_idx.unwrap_or(0),multiple) {
            multiples_iter.for_each(|local_multiple| *local_multiple=None)
        }
    }

    pub fn bump_seive(&mut self) -> bool{
        let new_start_val = self.seg_end()+Self::STEP;
        let new_end_val = new_start_val+Self::NUMS_PER_SEG;
        self.segmented_seive = Self::new_seive(new_start_val, self.range);
        self.num_of_loops+=1;
        self.last_primes_idx=None;
        return if new_end_val+Self::NUMS_PER_SEG<self.range {true} else {false}
    }

    pub fn new_seive(start:usize, end:usize) -> [Option<NonZeroUsize>;SEG_SIZE] {
        (start..end).step_by(Self::STEP).map(|num| NonZeroUsize::new(num)).collect_array_with_defaults() 
    }

    pub fn find_some(&mut self, start:usize) -> Option<usize> {
        self.segmented_seive.iter().skip(start).position(|num| num.is_some()).map(|relative_idx| start + relative_idx)
    }
}
