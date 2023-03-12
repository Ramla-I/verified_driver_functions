// use prusti_contracts::*;

// #[extern_spec]
// impl<T> alloc::vec::Vec<T> {
//     #[ensures(result.len() == 0)]
//     pub fn new() -> Vec<T>;
// }

// // I have to constrain T to be Copy because the old() operator expects it?
// #[extern_spec]
// impl<T: PartialEq + Copy, A: alloc::alloc::Allocator> alloc::vec::Vec<T, A> {
//     #[pure]
//     pub fn len(&self) -> usize;


//     #[ensures(result.is_some() ==> self.len() == old(self.len()) - 1)]
//     #[ensures(result.is_none() ==> self.len() == old(self.len()))]
//     #[ensures(result.is_some() ==>  forall (|i:usize| (0 <= i && i < self.len() - 1) ==> {
//         self[i] == old(self[i])
//     }))]
//     #[ensures(result.is_some() ==> old(self[self.len() - 1]) == result.unwrap())]
//     #[ensures(result.is_none() ==>  forall (|i:usize| (0 <= i && i < self.len()) ==> self[i] == old(self[i])))]
//     pub fn pop(&mut self) -> Option<T>;


//     #[ensures(self.len() == old(self.len()) + 1)]
//     #[ensures(forall (|i:usize| (0 <= i && i < self.len() - 1) ==> {
//         self[i] == old(self[i])
//     }))]
//     #[ensures( self[self.len() - 1] == value)]
//     pub fn push(&mut self, value: T);
// }

// #[extern_spec]
// impl<T, I: core::slice::SliceIndex<[T]>, A: alloc::alloc::Allocator> core::ops::Index<I> for Vec<T, A> {
//     fn index(&self, index: I) -> &Self::Output;
// }

// // #[trusted]
// #[pure]
// pub fn get<T>(vector: &Vec<T>, index: usize) -> &T {
//     &vector[index]
// }

use prusti_contracts::*;

// pub struct VecWrapperI32 {
//     v: Vec<i32>
// }

// impl VecWrapperI32 {
//     #[trusted]
//     #[pure]
//     #[ensures(result >= 0)]
//     pub fn len(&self) -> usize {
//         self.v.len()
//     }

//     /// A ghost function for specifying values stored in the vector.
//     #[trusted]
//     #[pure]
//     #[requires(0 <= index && index < self.len())]
//     pub fn lookup(&self, index: usize) -> i32 {
//         self.v[index]
//     }

//     #[trusted]
//     #[requires(0 <= index && index < self.len())]
//     #[ensures(*result == old(self.lookup(index)))]
//     #[after_expiry(
//         self.len() == old(self.len()) &&
//         self.lookup(index) == before_expiry(*result) &&
//         forall(
//             |i: usize| (0 <= i && i < self.len() && i != index) ==>
//             self.lookup(i) == old(self.lookup(i))
//         )
//     )]
//     pub fn index_mut(&mut self, index: usize) -> &mut i32 {
//         self.v.get_mut(index).unwrap()
//     }

//     #[trusted]
//     #[ensures(self.len() == old(self.len() + 1))]
//     // #[after_expiry(
//     //     self.len() == old(self.len() + 1) &&
//     //     self.lookup(self.len() - 1) == elem &&
//     //     forall(
//     //         |i: usize| (0 <= i && i < old(self.len())) ==>
//     //         self.lookup(i) == old(self.lookup(i))
//     //     )
//     // )]
//     pub fn push(&mut self, elem: i32) {
//         // self.v.push(elem)
//     }
// }

use crate::{option_spec::*, structs::PacketBufferS};

pub struct VecWrapper<T>{
    pub(crate) v: Vec<T>
}

impl<T> VecWrapper<T> {

    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        VecWrapper{ v: Vec::new() }
    }

    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(0 <= index && index < self.len())]
    pub fn index(&self, index: usize) -> &T {
        &self.v[index]
    }

    #[trusted]
    #[requires(0 <= index && index < self.len())]
    #[after_expiry(self.len() == old(self.len()))]
    pub fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.v[index]
    }
}

impl VecWrapper<PacketBufferS> {
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(forall (|i: usize| 0 <= i && i < old(self.len()) ==> {
        self.index(i).phys_addr.value() == old(self.index(i)).phys_addr.value()
    }))]
    #[after_expiry({
        let idx = self.len() - 1;
        self.index(idx).phys_addr.value() == value.phys_addr.value()
    })]
    pub fn push(&mut self, value: PacketBufferS) {
        self.v.push(value);
    }

    #[trusted]
    #[ensures(result.is_some() ==> self.len() == old(self.len()) - 1)]
    #[ensures(result.is_none() ==> self.len() == old(self.len()))]
    #[ensures(forall (|i: usize| 0 <= i && i < self.len() ==> {
        self.index(i).phys_addr.value() == old(self.index(i)).phys_addr.value()
    }))]
    #[ensures(result.is_some() ==> peek_option_ref(&result).phys_addr.value() == old(self.index(self.len() - 1)).phys_addr.value())]
    pub fn pop(&mut self) -> Option<PacketBufferS> {
        self.v.pop()
    }
}