use crate::nat::{_0, _1, Nat, Succ};

pub type Add<A, B> = <A as NatAdd<B>>::Out;
pub type Inc<A> = Add<A, _1>;

pub trait NatAdd<B: Nat> where Self: Nat {
    type Out: Nat;
    fn add(self, b: B) -> Self::Out;
}

impl <A: Nat> NatAdd<_0> for A {
    type Out = A;
    fn add(self, _: _0) -> Self::Out {
        self
    }
}

impl <A: Nat, B: Nat> NatAdd<Succ<B>> for A where A: NatAdd<B> {
    type Out = Succ<<A as NatAdd<B>>::Out>;
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