#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::cmp::Ordering;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::marker::PhantomData;
use super::*;
#[derive(Debug,Clone)]
pub struct Queue{
    customers: Vec<Customer>,
    pub capacity: usize,
}
impl Queue  {
    pub fn new() -> Self {
        return Queue {customers: Default::default(), capacity: 10};
    }
    pub fn size(&self) -> usize {
        self.customers.len()
    }
    pub fn insert( &mut self, ctx: &mut Context, customer: Customer) {
        let mut  server = ctx.get_servers();
        let mut sim = ctx.get_simulator();
        let t = sim.get_clock();
        if server.is_available() {
            server.insert( ctx, Some(customer));
        } else if self.customers.len() >= self.capacity  {
            println!("{:?} dropped  {:?} ", sim.get_clock(), customer);
        } else {
            self.customers.push(customer);
        }

    }
    pub fn remove(&mut self) -> Option<Customer>{
        self.customers.pop()
    }

}
#[test]
fn test_job_queue(){

    let mut queue :Queue = Queue::new();
    assert_eq!( queue.customers, vec![] );
}