use std::num::NonZeroUsize;
use std::iter::Peekable;
pub struct SegSeive<const SEG_SIZE: usize> {
    pub seive: [bool;SEG_SIZE],
    pub last_prime_idx:Option<usize>,
    pub num_of_loops:usize,
    pub range:usize
}

impl <const SEG_SIZE:usize>SegSeive<SEG_SIZE> {
    pub fn new(range:usize) -> Self{
        Self {
            seive:[true;SEG_SIZE],
            last_prime_idx:None,
            num_of_loops:0,
            range
        }
    }

    pub fn num_of_seive_bumps(&self) -> usize {
        (self.range-Self::START_NUM).div_ceil(Self::STEP*SEG_SIZE) //Div truncates to 0 
    }

    pub const START_NUM:usize =3;
    pub const STEP:usize =2;

    pub fn clear_seive(&mut self) {self.seive.fill(true);}

    pub fn bump_seive(&mut self) {
        self.num_of_loops+=1;
        self.last_prime_idx = None;
        self.clear_seive()
    }

    pub fn seg_start_globdex(&self) -> usize { self.num_of_loops*SEG_SIZE}
    pub fn seg_end_globdex(&self) -> usize { self.seg_start_globdex()+SEG_SIZE-1}
    
    pub fn guess_dex(&self, local_index:usize) -> usize {
        (self.seg_start_globdex()+local_index)*Self::STEP+Self::START_NUM
    }

    pub fn index(&self, local_index:usize) -> Option<NonZeroUsize> {
        if self.seive[local_index]==true {
            NonZeroUsize::new(self.guess_dex(local_index))
        } else {None}
    }

    pub fn find_set(&self, start:usize) -> Option<usize> {
        self.seive[start..].iter().position(|num| *num).map(|relative_idx| start + relative_idx)
    }

    pub fn global_value_to_global_idx(global_value:usize) -> usize {
        (global_value-Self::START_NUM)/Self::STEP
    }

    pub fn seg_start(&self) -> usize {  self.guess_dex(0) }
    pub fn seg_end(&self) -> usize {  self.seg_start() + Self::STEP*SEG_SIZE }
    pub fn is_value_in_seg(&self, value:usize) -> bool {
        value<=self.seg_end() && value>= self.seg_start()
    }

    pub fn is_global_index_in_seg(&self, global_idx:usize) -> bool {
        global_idx<=self.seg_end_globdex() && global_idx>=self.seg_start_globdex()
    }

    pub fn global_idx_to_local_idx(&self, global_idx:usize) -> Option<usize> {
        if self.is_global_index_in_seg(global_idx) {
            Some(global_idx-self.seg_start_globdex())
        } else {None}
    }

    pub fn local_index_to_global_index(&self,local_index:usize) -> usize {
        local_index+self.seg_start_globdex()
    }

    pub fn next_global_multiples_idx(global_idx:usize, multiple:usize) -> usize {
        let first_multiple_idx=Self::global_value_to_global_idx(multiple);
        let global_idx=global_idx.max(first_multiple_idx);
        let idx_difference = global_idx - first_multiple_idx;
        first_multiple_idx +idx_difference.div_ceil(multiple) * multiple
    }

    pub fn next_local_multiple(&self, local_index:usize, multiple:usize) -> Option<usize> {
        self.global_idx_to_local_idx(Self::next_global_multiples_idx(self.local_index_to_global_index(local_index),multiple))
    }

    pub fn mut_next_local_multiples_iter<'a>(&'a mut self, start:usize, multiple:usize) -> Option<Peekable<impl Iterator<Item = &'a mut bool>> > {
        if let Some(first_multiple_idx) = self.next_local_multiple(start,multiple) {
          Some(self.seive[first_multiple_idx..].iter_mut().step_by(multiple).peekable())
        } else {None}
    }

    pub fn remove_multiple(&mut self, multiple:usize) {
            if let Some(multiples_iter) = self.mut_next_local_multiples_iter(self.last_prime_idx.unwrap_or(0), multiple) {
                multiples_iter.for_each(|bol| *bol=false)
            }
    }

    pub fn remove_multiples_in_iter<'a>(&mut self, iterator: impl Iterator<Item = &'a usize>  ) {
        iterator.for_each(|multiple|  self.remove_multiple(*multiple));
    }
}