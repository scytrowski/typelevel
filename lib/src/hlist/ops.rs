use crate::hlist::{Cons, HList, Nil};
use crate::nat::{_0, Nat};
use crate::nat::ops::Inc;

pub type Head<L> = <L as HListHead>::Out;
pub type Tail<L> = <L as HListTail>::Out;
pub type PushBack<L, A> = <L as HListPushBack<A>>::Out;
pub type Append<A, B> = <A as HListAppend<B>>::Out;

pub trait HListHead where Self: HList {
    type Out;
    fn head(self) -> Self::Out;
}

impl <H, T: HList> HListHead for Cons<H, T> {
    type Out = H;
    fn head(self) -> Self::Out {
        self.0
    }
}

pub trait HListTail where Self: HList {
    type Out: HList;
    fn tail(self) -> Self::Out;
}

impl <H, T: HList> HListTail for Cons<H, T> {
    type Out = T;
    fn tail(self) -> Self::Out {
        self.1
    }
}

pub trait HListPushBack<A> where Self: HList {
    type Out: HList;
    fn push_back(self, a: A) -> Self::Out;
}

impl <A> HListPushBack<A> for Nil {
    type Out = Cons<A, Nil>;
    fn push_back(self, a: A) -> Self::Out {
        Nil.push_front(a)
    }
}

impl <H, T: HList, A> HListPushBack<A> for Cons<H, T> where T: HListPushBack<A> {
    type Out = Cons<H, <T as HListPushBack<A>>::Out>;
    fn push_back(self, a: A) -> Self::Out {
        Cons(self.0, self.1.push_back(a))
    }
}

pub trait HListAppend<B: HList> where Self: HList {
    type Out: HList;
    fn append(self, b: B) -> Self::Out;
}

impl <B: HList> HListAppend<B> for Nil {
    type Out = B;
    fn append(self, b: B) -> Self::Out {
        b
    }
}

impl <H, T: HList, B: HList> HListAppend<B> for Cons<H, T> where T: HListAppend<B> {
    type Out = Cons<H, <T as HListAppend<B>>::Out>;
    fn append(self, b: B) -> Self::Out {
        Cons(self.0, self.1.append(b))
    }
}

pub trait Size where Self: HList {
    type Out: Nat;
    fn size(&self) -> Self::Out;
}

impl Size for Nil {
    type Out = _0;
    fn size(&self) -> Self::Out {
        _0
    }
}

impl <H, T: HList> Size for Cons<H, T> where T: Size {
    type Out = Inc<<T as Size>::Out>;
    fn size(&self) -> Self::Out {
        self.1.size().inc()
    }
}