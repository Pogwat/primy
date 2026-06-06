use std::num::NonZeroUsize;
struct SegSeive<const SEG_SIZE: usize> {
    seive: [bool;SEG_SIZE],
    current_prime_idx:Option<usize>,
    num_of_loops:usize,
    range:usize
}

impl <const SEG_SIZE:usize>SegSeive<SEG_SIZE> {
    fn new(range:usize) -> Self{
        Self {
        seive:[true;SEG_SIZE],
        current_prime_idx:None,
        num_of_loops:0,
        range
        }
    }

    pub const START_NUM:usize =3;
    pub const STEP:usize =2;

    fn clear_seive(&mut self) {self.seive.fill(true);}

    fn bump_seive(&mut self) {
        self.num_of_loops+=1;
        self.current_prime_idx = None;
        self.clear_seive()
    }
    
    fn index_to_value(&self, local_index:usize) -> Option<NonZeroUsize> {
        return if self.seive[local_index]==true {
            NonZeroUsize::new(self.num_of_loops*Self::STEP+Self::START_NUM+local_index)
        } else {None}
    }

    fn find_set(&self, start:usize) -> Option<usize> {
        self.seive[start..].iter().position(|num| *num).map(|relative_idx| start + relative_idx)
    }

    pub fn seg_start(&self) -> usize {Self::global_idx_to_value(self.seg_start_globdex())}
    pub fn seg_end(&self) -> usize {self.seg_start()+SEG_SIZE*Self::STEP}
    pub fn is_value_within_seg(&self, value:usize) -> bool{ value <= self.seg_end() && value >= self.seg_start() }

    pub fn seg_start_globdex(&self) -> usize {self.num_of_loops*SEG_SIZE}
    pub fn seg_end_globdex(&self) -> usize {self.seg_start_globdex()+SEG_SIZE-1}
    pub fn is_global_idx_within_seg(&self, global_idx:usize) -> bool {
        global_idx>= self.seg_start_globdex() && global_idx<= self.seg_end_globdex()
    }

    pub fn global_idx_to_value(global_idx:usize) -> usize {global_idx*Self::STEP+Self::START_NUM}
    pub fn global_idx_to_local_idx(&self, global_idx:usize) -> Option<usize>{
        return if self.is_global_idx_within_seg(global_idx) {
            Some(global_idx-self.seg_start_globdex())
        } else {None}
    }

    pub fn local_value_to_local_idx(&self,local_value:usize) -> usize{ (local_value-self.seg_start())/Self::STEP }
    pub fn local_idx_to_global_idx(&self, local_idx:usize) -> usize {self.seg_start_globdex()+local_idx}

    pub fn global_value_to_global_idx(global_value:usize) -> usize {(global_value-Self::START_NUM)/Self::STEP}
    pub fn global_value_to_local_idx(&self, global_value:usize) -> Option<usize> {
        return if self.is_value_within_seg(global_value) {
            Some(self.local_value_to_local_idx(global_value))
        } else {None}
    }

    pub fn next_global_multiples_idx(global_idx:usize, multiple:usize) -> usize {
        let first_multiple_idx=Self::global_value_to_global_idx(multiple);
        let global_idx=global_idx.max(first_multiple_idx);
        let idx_difference = global_idx - first_multiple_idx;
        first_multiple_idx +idx_difference.div_ceil(multiple) * multiple
    }

    pub fn next_local_multiples_idx(&self,local_idx:usize, multiple:usize) -> Option<usize> {
        self.global_idx_to_local_idx(Self::next_global_multiples_idx(self.local_idx_to_global_idx(local_idx), multiple))
    }
}