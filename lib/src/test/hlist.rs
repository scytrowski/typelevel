use crate::hlist::{Cons, HList, Nil};
use crate::hlist::ops::{HListAppend, HListHead, HListPushBack, HListTail, Size};
use crate::nat::{_0, _3};

#[test]
fn get_head_of_single_cons() {
    assert_eq!(
        Nil.push_front(5).head(),
        5
    );
}

#[test]
fn get_head_of_multiple_cons() {
    assert_eq!(
        Nil.push_front("abc").push_front("def").push_front("ghi").head(),
        "ghi"
    );
}

#[test]
fn get_tail_of_single_cons() {
    assert_eq!(
        Nil.push_front(false).tail(),
        Nil
    );
}

#[test]
fn get_tail_of_multiple_cons() {
    assert_eq!(
        Nil.push_front("abc").push_front("def").push_front("ghi").tail(),
        Nil.push_front("abc").push_front("def")
    );
}

#[test]
fn push_back_to_nil() {
    assert_eq!(
        Nil.push_back(15),
        Cons(15, Nil)
    );
}

#[test]
fn push_back_to_single_cons() {
    assert_eq!(
        Nil.push_front(20).push_back(50),
        Nil.push_front(50).push_front(20)
    );
}

#[test]
fn push_back_to_multiple_cons() {
    assert_eq!(
        Nil.push_front("a").push_front("b").push_front("c").push_back("d"),
        Nil.push_front("d").push_front("a").push_front("b").push_front("c")
    );
}

#[test]
fn append_nil_to_nil() {
    assert_eq!(Nil.append(Nil), Nil);
}

#[test]
fn append_nil_to_single_cons() {
    assert_eq!(
        Nil.push_front(1337).append(Nil),
        Nil.push_front(1337)
    );
}

#[test]
fn append_nil_to_multiple_cons() {
    assert_eq!(
        Nil.push_front(1).push_front(2).push_front(3).append(Nil),
        Nil.push_front(1).push_front(2).push_front(3)
    );
}

#[test]
fn append_single_cons_to_nil() {
    assert_eq!(
        Nil.append(Nil.push_front("abc")),
        Nil.push_front("abc")
    );
}

#[test]
fn append_multiple_cons_to_nil() {
    assert_eq!(
        Nil.append(Nil.push_front("abc").push_front("def").push_front("ghi")),
        Nil.push_front("abc").push_front("def").push_front("ghi")
    );
}

#[test]
fn append_single_cons_to_single_cons() {
    assert_eq!(
        Nil.push_front(1).append(Nil.push_front(2)),
        Nil.push_front(2).push_front(1)
    );
}

#[test]
fn append_single_cons_to_multiple_cons() {
    assert_eq!(
        Nil.push_front(1).push_front(2).push_front(3).append(Nil.push_front(4)),
        Nil.push_front(4).push_front(1).push_front(2).push_front(3)
    );
}

#[test]
fn append_multiple_cons_to_single_cons() {
    assert_eq!(
        Nil.push_front("a").append(Nil.push_front("b").push_front("c").push_front("d")),
        Nil.push_front("b").push_front("c").push_front("d").push_front("a")
    );
}
#[test]
fn append_multiple_cons_to_multiple_cons() {
    assert_eq!(
        Nil.push_front("a").push_front("b").push_front("c").append(
            Nil.push_front("d").push_front("e").push_front("f")
        ),
        Nil
            .push_front("d").push_front("e").push_front("f")
            .push_front("a").push_front("b").push_front("c")
    )
}

#[test]
fn compute_size_of_nil() {
    assert_eq!(Nil.size(), _0);
}

#[test]
fn compute_size_of_cons() {
    assert_eq!(
        Nil.push_front(1).push_front("a").push_front(0.15).size(),
        _3
    );
}