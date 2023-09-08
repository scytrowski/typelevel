use crate::hlist;
use crate::hlist::{HList, Nil};
use crate::hlist::ops::{HListAppend, HListHead, HListPushBack, HListTail, Size};
use crate::nat::{_0, _3};

#[test]
fn create_nil() {
    assert_eq!(hlist![], Nil);
}

#[test]
fn create_single_cons() {
    assert_eq!(hlist![79], Nil.push_front(79));
}

#[test]
fn create_multiple_cons() {
    assert_eq!(
        hlist![19, "test", false],
        Nil.push_front(false).push_front("test").push_front(19)
    );
}

#[test]
fn get_head_of_single_cons() {
    assert_eq!(hlist![5].head(), 5);
}

#[test]
fn get_head_of_multiple_cons() {
    assert_eq!(hlist!["ghi", "def", "abc"].head(), "ghi");
}

#[test]
fn get_tail_of_single_cons() {
    assert_eq!(hlist![false].tail(), Nil);
}

#[test]
fn get_tail_of_multiple_cons() {
    assert_eq!(hlist!["ghi", "def", "abc"].tail(), hlist!["def", "abc"]);
}

#[test]
fn push_back_to_nil() {
    assert_eq!(Nil.push_back(15), hlist![15]);
}

#[test]
fn push_back_to_single_cons() {
    assert_eq!(hlist![20].push_back(50), hlist![20, 50]);
}

#[test]
fn push_back_to_multiple_cons() {
    assert_eq!(
        hlist!["c", "b", "a"].push_back("d"),
        hlist!["c", "b", "a", "d"]
    );
}

#[test]
fn append_nil_to_nil() {
    assert_eq!(Nil.append(Nil), Nil);
}

#[test]
fn append_nil_to_single_cons() {
    assert_eq!(hlist![1337].append(Nil), hlist![1337]);
}

#[test]
fn append_nil_to_multiple_cons() {
    assert_eq!(hlist![3, 2, 1].append(Nil), hlist![3, 2, 1]);
}

#[test]
fn append_single_cons_to_nil() {
    assert_eq!(Nil.append(hlist!["abc"]), hlist!["abc"]);
}

#[test]
fn append_multiple_cons_to_nil() {
    assert_eq!(Nil.append(hlist!["ghi", "def", "abc"]), hlist!["ghi", "def", "abc"]);
}

#[test]
fn append_single_cons_to_single_cons() {
    assert_eq!(hlist![1].append(hlist![2]), hlist![1, 2]);
}

#[test]
fn append_single_cons_to_multiple_cons() {
    assert_eq!(hlist![3, 2, 1].append(hlist![4]), hlist![3, 2, 1, 4]);
}

#[test]
fn append_multiple_cons_to_single_cons() {
    assert_eq!(
        hlist!["a"].append(hlist!["d", "c", "b"]),
        hlist!["a", "d", "c", "b"]
    );
}
#[test]
fn append_multiple_cons_to_multiple_cons() {
    assert_eq!(
        hlist!["c", "b", "a"].append(hlist!["f", "e", "d"]),
        hlist!["c", "b", "a", "f", "e", "d"]
    );
}

#[test]
fn compute_size_of_nil() {
    assert_eq!(Nil.size(), _0);
}

#[test]
fn compute_size_of_cons() {
    assert_eq!(hlist![0.15, "a", 1].size(), _3);
}