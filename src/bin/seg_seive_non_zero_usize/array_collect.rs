pub trait ArrayCollectExt: Iterator + Sized {
    // The method takes a const generic size parameter `SIZE`
    fn collect_array<const SIZE: usize>(&mut self) -> Option<[Self::Item; SIZE]> {
        let mut error = false;
        
        let array = std::array::from_fn(|_| {
            match self.next() {
                Some(val) => val,
                None => {
                    error = true;
                    // Fallback to satisfy from_fn signature if iterator is short
                    unsafe { std::mem::zeroed() }
                }
            }
        });

        if error { None } else { Some(array) }
    }

        fn collect_array_with_defaults<const SIZE: usize>(&mut self) -> [Self::Item; SIZE] 
        where <Self as Iterator>::Item: Default    
        {
        std::array::from_fn(|_| self.next().unwrap_or_default() )
    }
}

// 2. Blanket implement it for ALL iterators automatically
impl<I: Iterator> ArrayCollectExt for I {}