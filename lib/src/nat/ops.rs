use crate::nat::{_0, Nat, Succ};

pub trait Add<B: Nat> where Self: Nat {
    type Out: Nat;
    fn add(self, b: B) -> Self::Out;
}

impl <A: Nat> Add<_0> for A {
    type Out = A;
    fn add(self, _: _0) -> Self::Out {
        self
    }
}

impl <A: Nat, B: Nat> Add<Succ<B>> for A where A: Add<B> {
    type Out = Succ<<A as Add<B>>::Out>;
    fn add(self, b: Succ<B>) -> Self::Out {
        Succ(self.add(b.0))
    }
}

pub trait ToNumber where Self: Nat {
    fn to_number(&self) -> u32;
}

impl ToNumber for _0 {
    fn to_number(&self) -> u32 {
        0
    }
}

impl <N: Nat> ToNumber for Succ<N> where N: ToNumber {
    fn to_number(&self) -> u32 {
        self.0.to_number() + 1
    }
}