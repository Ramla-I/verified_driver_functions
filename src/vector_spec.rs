use prusti_contracts::*;
use crate::{option_spec::*, structs::PacketBuffer};

pub struct VecWrapper<T: PartialEq>{
    pub(crate) v: Vec<T>
}

impl<T: PartialEq> VecWrapper<T> {

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

    // #[trusted]
    // #[requires(0 <= index && index < self.len())]
    // #[after_expiry(self.len() == old(self.len()))]
    // pub fn index_mut(&mut self, index: usize) -> &mut T {
    //     &mut self.v[index]
    // }
}

impl VecWrapper<PacketBuffer> {
    /// Ideally this should be a generic function, but I get the error
    /// "[Prusti: invalid specification] use of impure function "std::cmp::PartialEq::eq" in pure code is not allowed"
    /// even though I implemented PartialEq for PacketBuffer and declared it as a pure fn
    #[trusted]
    #[requires(0 <= index && index < self.len())]
    // #[after_expiry(self.len() == old(self.len()))]
    #[after_expiry(
        self.len() == old(self.len()) &&
        self.index(index).phys_addr.value() == before_expiry(result).phys_addr.value() &&
        forall(
            |i: usize| (0 <= i && i < self.len() && i != index) ==>
            self.index(i).phys_addr.value() == old(self.index(i).phys_addr.value())
        )
    )]
    pub fn index_mut(&mut self, index: usize) -> &mut PacketBuffer {
        &mut self.v[index]
    }

    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(forall (|i: usize| 0 <= i && i < old(self.len()) ==> {
        self.index(i).phys_addr.value() == old(self.index(i)).phys_addr.value()
    }))]
    #[after_expiry({
        let idx = self.len() - 1;
        self.index(idx).phys_addr.value() == value.phys_addr.value()
    })]
    pub fn push(&mut self, value: PacketBuffer) {
        self.v.push(value);
    }

    #[trusted]
    #[ensures(result.is_some() ==> self.len() == old(self.len()) - 1)]
    #[ensures(result.is_none() ==> self.len() == old(self.len()))]
    #[ensures(forall (|i: usize| 0 <= i && i < self.len() ==> {
        self.index(i).phys_addr.value() == old(self.index(i)).phys_addr.value()
    }))]
    #[ensures(result.is_some() ==> peek_option_ref(&result).phys_addr.value() == old(self.index(self.len() - 1)).phys_addr.value())]
    pub fn pop(&mut self) -> Option<PacketBuffer> {
        self.v.pop()
    }
}
