use prusti_contracts::*;
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

impl VecWrapper<usize> {
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(forall (|i: usize| 0 <= i && i < old(self.len()) ==> {
        self.index(i) == old(self.index(i))
    }))]
    #[after_expiry({
        let idx = self.len() - 1;
        *self.index(idx) == value
    })]
    pub fn push(&mut self, value: usize) {
        self.v.push(value);
    }

    #[trusted]
    #[ensures(result.is_some() ==> self.len() == old(self.len()) - 1)]
    #[ensures(result.is_none() ==> self.len() == old(self.len()))]
    #[ensures(forall (|i: usize| 0 <= i && i < self.len() ==> {
        self.index(i) == old(self.index(i))
    }))]
    #[ensures(result.is_some() ==> peek_option_ref(&result) == old(self.index(self.len() - 1)))]
    pub fn pop(&mut self) -> Option<usize> {
        self.v.pop()
    }
}