pub mod ops;

pub trait Nat {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct _0;
impl Nat for _0 {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Succ<N: Nat>(N);
impl <N: Nat> Nat for Succ<N> {}

pub type _1 = Succ<_0>;
pub const _1: _1 = Succ(_0);

pub type _2 = Succ<_1>;
pub const _2: _2 = Succ(_1);

pub type _3 = Succ<_2>;
pub const _3: _3 = Succ(_2);

pub type _4 = Succ<_3>;
pub const _4: _4 = Succ(_3);

pub type _5 = Succ<_4>;
pub const _5: _5 = Succ(_4);

pub type _6 = Succ<_5>;
pub const _6: _6 = Succ(_5);

pub type _7 = Succ<_6>;
pub const _7: _7 = Succ(_6);

pub type _8 = Succ<_7>;
pub const _8: _8 = Succ(_7);

pub type _9 = Succ<_8>;
pub const _9: _9 = Succ(_8);