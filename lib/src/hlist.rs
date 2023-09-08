use crate::private::Sealed;

pub mod ops;

pub trait HList : Sealed {
    fn push_front<A>(self, a: A) -> Cons<A, Self> where A: Sized, Self: Sized {
        Cons(a, self)
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Nil;
impl Sealed for Nil {}
impl HList for Nil {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Cons<H, T: HList>(pub H, pub T);
impl <H, T: HList> Sealed for Cons<H, T> {}
impl <H, T: HList> HList for Cons<H, T> {}

