// works with command line release versions, but not with the vscode extension

extern crate prusti_contracts;
extern crate core;

use prusti_contracts::*;
use core::ops::RangeInclusive;


pub struct StaticArray  {
	pub(crate) arr: [Option<RangeInclusive<usize>>; 2],
    pub(crate) curr_idx: usize
}

impl StaticArray {
    #[ensures(result.curr_idx <= result.arr.len())]
    pub const fn new() -> Self {
        StaticArray {
            arr: [None, None],
            curr_idx: 0
        }
    }

    #[pure]
    #[requires(self.curr_idx <= self.arr.len())]
    #[requires(0 <= index && index < self.curr_idx)]
    // #[requires(0 <= index && index < self.arr.len())]
    pub fn lookup(&self, index: usize) -> &Option<RangeInclusive<usize>> {
        &self.arr[index]
    }

	pub fn push(&mut self, value: RangeInclusive<usize>) -> Option<usize> {
        if self.curr_idx >= self.arr.len() {
            None
        } else {
            self.arr[0] = Some(value);
            self.curr_idx += 1;
            Some(self.curr_idx)
        }
	}
}

fn main() {
    
}