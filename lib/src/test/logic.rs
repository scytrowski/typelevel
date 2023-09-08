use crate::logic::{False, True};
use crate::logic::ops::{AtomAnd, AtomNot, AtomOr, ToBool};

#[test]
fn convert_false_to_bool() {
    assert_eq!(False.to_bool(), false);
}

#[test]
fn convert_true_to_bool() {
    assert_eq!(True.to_bool(), true);
}

#[test]
fn not_false_is_true() {
    assert_eq!(False.not(), True);
}

#[test]
fn not_true_is_false() {
    assert_eq!(True.not(), False);
}

#[test]
fn false_and_false_is_false() {
    assert_eq!(False.and(False), False);
}

#[test]
fn false_and_true_is_false() {
    assert_eq!(False.and(True), False);
}

#[test]
fn true_and_false_is_false() {
    assert_eq!(True.and(False), False);
}

#[test]
fn true_and_true_is_true() {
    assert_eq!(True.and(True), True);
}

#[test]
fn false_or_false_is_false() {
    assert_eq!(False.or(False), False);
}

#[test]
fn false_or_true_is_true() {
    assert_eq!(False.or(True), True);
}

#[test]
fn true_or_false_is_true() {
    assert_eq!(True.or(False), True);
}

#[test]
fn true_or_true_is_true() {
    assert_eq!(True.or(True), True);
}