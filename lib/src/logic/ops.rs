use crate::logic::{Atom, False, True};

pub type Not<A> = <A as AtomNot>::Out;

pub trait AtomNot where Self: Atom {
    type Out: Atom;
    fn not(&self) -> Self::Out;
}

impl AtomNot for False {
    type Out = True;
    fn not(&self) -> Self::Out {
        True
    }
}

impl AtomNot for True {
    type Out = False;
    fn not(&self) -> Self::Out {
        False
    }
}

pub type And<A, B> = <A as AtomAnd<B>>::Out;

pub trait AtomAnd<B: Atom> where Self: Atom {
    type Out: Atom;
    fn and(&self, b: B) -> Self::Out;
}

impl <B: Atom> AtomAnd<B> for False {
    type Out = False;
    fn and(&self, _: B) -> Self::Out {
        False
    }
}

impl <B: Atom> AtomAnd<B> for True {
    type Out = B;
    fn and(&self, b: B) -> Self::Out {
        b
    }
}

pub type Or<A, B> = <A as AtomOr<B>>::Out;

pub trait AtomOr<B: Atom> where Self: Atom {
    type Out: Atom;
    fn or(&self, b: B) -> Self::Out;
}


impl <B: Atom> AtomOr<B> for False {
    type Out = B;
    fn or(&self, b: B) -> Self::Out {
        b
    }
}

impl <B: Atom> AtomOr<B> for True {
    type Out = True;
    fn or(&self, _: B) -> Self::Out {
        True
    }
}

pub trait ToBool where Self: Atom {
    fn to_bool(&self) -> bool;
}

impl ToBool for False {
    fn to_bool(&self) -> bool {
        false
    }
}

impl ToBool for True {
    fn to_bool(&self) -> bool {
        true
    }
}
