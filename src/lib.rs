// works with command line release versions, but not with the vscode extension
extern crate prusti_contracts;
extern crate core;
extern crate alloc;


use prusti_contracts::*;
// mod vector_spec;
// mod structs;
mod option_spec;
mod result_spec;
// mod tx_rx;
// mod filter;

// use vector_spec::*;
// use structs::*;
use option_spec::*;
use result_spec::*;

// #[ensures(result == pages.len() - 1)]
// #[ensures(0 <= result && result < pages.len())]
// fn test(mut pages: &mut VecWrapper<i32>, elem: i32) -> usize {
//     pages.push(elem);
//     pages.len() - 1 
// }


fn main(){}