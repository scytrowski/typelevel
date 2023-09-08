use crate::private::Sealed;

pub mod ops;

pub trait Atom : Sealed {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct True;
impl Sealed for True {}
impl Atom for True {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct False;
impl Sealed for False {}
impl Atom for False {}