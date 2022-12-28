

// test suite for seq! macro
use crate::misc::sequence::{sequencer::Sequence, sequence_state::SequenceState};
use crate::{seq, map};
use std::any::Any;

#[test]
fn test_seq() {
    let mut seq = seq!(|x: &mut i32| {
            *x = *x + 1;
            return x.pow(2);
        });
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(1));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(4));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(9));
}
#[test]
fn test_two_params() {
    let mut seq = seq!(|x: &mut i32, y: &mut i32| {
            *x = *x + 1;
            *y = *y + 1;
            return x.pow(2) + y.pow(2);
        });
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(2));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(8));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some(18));
}
#[test]
fn test_complex() {
    let mut seq = seq!(|x: &mut i32, y: &mut i32| {
            *x = *x + 1;
            *y = *y + 1;
            return format!("{}: {}", x.pow(2), y.pow(2));
        });
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some("1: 1".to_string()));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some("4: 4".to_string()));
    let ret = seq.next();
    println!("{:?}", ret);
    assert_eq!(ret, Some("9: 9".to_string()));
}