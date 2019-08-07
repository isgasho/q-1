#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::Cell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
// use crate::simulator::AbstractSimulator;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use std::usize;

use super::*;
use super::base::*;
use super::simulator::*;

// use std::borrow::ToOwned;

pub trait AbstractEvent : Clocked{
    // type S: AbstractSimulator;
    fn execute(&mut self, ctx: &mut Context);

}
type EventId = u64;
#[derive(Debug,Clone,Copy )]
pub enum EventType {
    ServerEvent,
    GeneratorEvent
}
#[derive(Debug,Clone,Copy )]
pub struct Event {
    id: EventId,
    clock: Clock,
    etype: EventType
}

//
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.clock.cmp(&self.clock)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.get_clock().cmp(&self.get_clock()))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.get_clock() == other.get_clock()
    }
}
impl Eq for Event {

}

impl AbstractEvent for Event {
    fn execute(&mut self, ctx : &mut Context) {
        // info!("Running Event");
        match self.etype {
            EventType::ServerEvent => {
                let mut server = ctx.get_servers();
                server.execute(ctx);
            },
            EventType::GeneratorEvent =>{
                let mut generator = ctx.get_generator();
                generator.execute(ctx);
            }
        }
    }

}
impl Clocked for Event{
    fn get_clock(&self) -> Clock {
        self.clock
    }
    fn set_clock(&mut self, t: Clock) {
        self.clock = t;
    }
}
impl Event{
    pub fn new( etype: EventType, clock: Clock) -> Self{
        let id = 0;
        Event {id, clock, etype}
    }
}

#[derive(Debug)]
pub struct EventQueue {
    pub elements: BinaryHeap<Event>
}

impl EventQueue {
    pub fn insert(&mut self, t: Event) {
        self.elements.push(t);
    }
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn remove_first(&mut self) -> Option<Event> {
        self.elements.pop()
    }
    pub fn new() -> Self {
        EventQueue { elements: Default::default() }
    }

}



#[test]
fn test_event() {
}

#[test]
fn test_event_queue() {
}