use crate::nat::{_0, _1, _2, _5, _6, _7, _8};
use crate::nat::ops::{NatAdd, ToNumber};

#[test]
fn convert_0_to_number() {
    assert_eq!(_0.to_number(), 0)
}

#[test]
fn convert_successor_to_number() {
    assert_eq!(_1.to_number(), 1)
}

#[test]
fn convert_nested_successor_to_number() {
    assert_eq!(_5.to_number(), 5)
}

#[test]
fn add_0_to_0() {
    assert_eq!(_0.add(_0), _0)
}

#[test]
fn add_successor_to_0() {
    assert_eq!(_0.add(_6), _6)
}

#[test]
fn add_0_to_successor() {
    assert_eq!(_8.add(_0), _8)
}

#[test]
fn add_successor_to_successor() {
    assert_eq!(_2.add(_5), _7)
}