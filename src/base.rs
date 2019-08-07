#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use super::event::*;

#[derive(PartialEq, Ord,Eq, PartialOrd, Clone, Copy)]
pub struct Clock(i64);

impl From<i64> for Clock {
    fn from(t: i64) -> Self {
        Clock(t as i64)
    }
}


impl Clock {
    pub fn set_raw(&mut self, v: i64) {
        self.0 = v;
    }
    pub fn get_raw(&self) -> i64 {
        self.0
    }
}

impl std::ops::Add for Clock {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from(self.0 + other.0)
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{:010.03}", self.0)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Customer(u64);

impl Customer {
    pub fn new(id: u64) -> Self {
        Customer(id)
    }
}

impl std::fmt::Debug for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[customor {:04}]", self.0)
    }
}


#[test]
fn test_customer() {
    let c0 = Customer(10);
    assert_eq!(format!("{:?}", c0), "[customor 0010]")
}

pub trait Empty {
    fn empty() -> Self;
}
pub trait Clocked{
    fn now(&self) -> Clock {
        self.get_clock()
    }
    fn get_clock(&self) -> Clock ;
    fn set_clock(&mut self, t: Clock);
}
