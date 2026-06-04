use std::iter::Peekable;
use std::ops::Range;
use crate::array_collect::ArrayCollectExt;

pub struct SegmentedSeive<const SEG_SIZE:  usize > {
    pub segmented_seive : [Option<usize>;SEG_SIZE],
    pub current_idx:usize,
    pub range:usize,
    pub num_of_loops:usize
}

impl <const SEG_SIZE:  usize> SegmentedSeive<SEG_SIZE> {

    pub const FIRST_START_NUM:usize=3;
    pub const SEG_SIZE:usize = SEG_SIZE;
    pub const STEP:usize = 2;

    pub fn new(range:usize) -> Self {
        let seg_end_num:usize = Self::FIRST_START_NUM+Self::SEG_SIZE*Self::STEP;
        Self {
            segmented_seive: Self::new_seive(Self::FIRST_START_NUM),
            current_idx:0,
            range,
            num_of_loops:0
        }
    }

    //STEP BY MULTIPLES ARE BROKEN!!!!!!, NEED TO FIX

    pub fn guess_dex(&self,index:usize)-> usize {(self.num_of_loops*SEG_SIZE+index)*Self::STEP+Self::FIRST_START_NUM}
    pub fn seg_start(&self) -> usize {self.guess_dex(0)}
    pub fn seg_end(&self) -> usize {self.guess_dex(self.segmented_seive.len()-1)}
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

    pub fn global_value_to_global_idx(&self, global_value:usize) -> usize {(global_value-Self::FIRST_START_NUM)/Self::STEP}
    pub fn global_value_to_local_idx(&self, global_value:usize) -> Option<usize> {
        return if self.is_value_within_seg(global_value) {
            Some(self.local_value_to_local_idx(global_value))
        } else {None}
    }

    pub fn upper_multiple_idx_from_local_idx(&self, multiple:usize, start_local_idx:Option<usize>) -> Option<usize> {
        let start_value = self.guess_dex(start_local_idx.unwrap_or(0));
        let next_multiple_value = start_value.next_multiple_of(multiple);
        let next_upper_multiple_value = if next_multiple_value == start_value {
            start_value+multiple
        } else {next_multiple_value};
        self.global_value_to_local_idx(next_upper_multiple_value)
    }

    pub fn upper_multiples_mut_iter(&mut self,multiple:usize,start:Option<usize> ) -> Option<impl Iterator<Item = &mut Option<usize>>> {
        return if let Some(start_idx) = self.upper_multiple_idx_from_local_idx(multiple,start) {
            let segment_starting_at_upper_multiple = &mut self.segmented_seive[(start_idx..)];
            Some(segment_starting_at_upper_multiple.iter_mut().step_by(multiple))
        } else {None}
        }


    pub fn mut_multiple_iter<'a>(&'a mut self, multiple: usize, start: usize) -> Option<Peekable<impl Iterator<Item = &'a mut Option<usize>> + 'a>> {
        let value = self.guess_dex(start);
        if let Some(start_multiple_idx) = self.global_value_to_local_idx(value.next_multiple_of(multiple)) {
            let segment = &mut self.segmented_seive[start_multiple_idx..];
            Some(segment.iter_mut().step_by(multiple).peekable())
        } else {None}
    }

    pub fn remove_all_multiples_in_iter<'a>(&mut self, iterator:impl Iterator<Item = &'a usize>) {
        for multiple in iterator {
            if let Some(seg_multiples_iter) = self.mut_multiple_iter(*multiple, self.current_idx) {
                seg_multiples_iter.for_each(|multiple_in_seive| *multiple_in_seive=None)
            };
        }; 
    }

    pub fn bump_seive(&mut self) {
        let new_last = self.seg_end()+SEG_SIZE*Self::STEP;
        if new_last<=self.range  {
            self.segmented_seive= Self::new_seive(self.seg_end()+Self::STEP)

        }
    }

    pub fn new_seive(start:usize) -> [Option<usize>;SEG_SIZE] {
        (start..).step_by(Self::STEP).map(|num| Some(num as usize)).collect_array().unwrap()   
    }

    pub fn find_some(&mut self, start:usize) -> Option<usize> {
        self.segmented_seive.iter().skip(start).position(|num| num.is_some()).map(|relative_idx| start + relative_idx)
    }

    pub fn ranges_global_idx(&self) -> usize {self.global_value_to_global_idx(self.range - self.range%2)}
}
