#[cfg(test)]
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use crate::private::Sealed;

pub mod ops;

pub trait HList : Sealed {
    fn push_front<A>(self, a: A) -> Cons<A, Self> where A: Sized, Self: Sized {
        Cons(a, self)
    }
}

#[derive(PartialEq)]
pub struct Nil;
impl Sealed for Nil {}
impl HList for Nil {}
impl Display for Nil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Nil")
    }
}
#[cfg(test)]
impl Debug for Nil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Nil as Display>::fmt(self, f)
    }
}

#[derive(PartialEq)]
pub struct Cons<H, T: HList>(pub H, pub T);
impl <H, T: HList> Sealed for Cons<H, T> {}
impl <H, T: HList> HList for Cons<H, T> {}
impl <H, T: HList> Display for Cons<H, T> where H: Display, T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} :: {}", self.0, self.1))
    }
}
#[cfg(test)]
impl <H, T: HList> Debug for Cons<H, T> where H: Debug, T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} :: {:?}", self.0, self.1))
    }
}

#[macro_export]
macro_rules! hlist {
    () => {
        crate::hlist::Nil
    };
    ( $h: expr $( , $t: expr )* ) => {
        crate::hlist::Cons($h, hlist!($( $t ),*))
    }
}