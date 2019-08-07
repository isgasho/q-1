#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cmp::Ordering;

use super::base::*;
use super::event::*;
use super::generator::*;
use super::server::*;
use super::queue::*;
use crate::event::AbstractEvent;
use std::marker::PhantomData;

pub struct Context<'a , T = i32>{
    pub simulator: *mut Simulator,
    pub servers : *mut Server,
    pub generator: *mut Generator,
    pub events: *mut EventQueue,
    pub queue: *mut Queue,
    pub _marker:  PhantomData<&'a T>,
}
impl  <'a, T> Context<'a, T>{
    pub fn get_simulator(&self) -> &'a mut Simulator {
        // Use a reborrow instead
        let ptr =  unsafe { &mut *self.simulator };
        ptr

    }
    pub fn get_servers(&self) -> &'a mut Server {
        let ptr =  unsafe { &mut *self.servers };
        ptr

    }
    pub fn get_generator(&self) -> &'a mut Generator {
        let ptr =  unsafe { &mut *self.generator };
        ptr

    }
    pub fn get_events(&self) -> &'a mut EventQueue {
        let ptr =  unsafe { &mut *self.events };
        ptr

    }
    pub fn get_queue(&self) -> &'a mut Queue {
        let ptr =  unsafe { &mut *self.queue };
        ptr

    }
}
pub trait AbstractSimulator {
    fn insert(&mut self, ctx: &mut Context, e: Event);
    fn cancel(&mut self, e: Event) {
        unimplemented!()
    }
}

pub struct Simulator {
    clock: Clock,

}

impl AbstractSimulator for Simulator {

    fn insert(&mut self, ctx: &mut Context, event: Event) {
        let events = ctx.get_events();
        events.insert( event);
    }
}
impl Clocked for Simulator {
    fn get_clock(&self) -> Clock {
        self.clock
    }
    fn set_clock(&mut self, t: Clock) {
        self.clock = t;
    }
}
impl Simulator {
    pub fn new() -> Simulator {
        Simulator {  clock: Clock::from(0)}
    }
}

pub fn do_all_events(ctx: &mut  Context) -> (i64,i64,i64) {
    loop {
        let event =  ctx.get_events().remove_first();
        match event {
            Some(mut e) => {
                let t = e.get_clock() ;
                ctx.get_simulator().set_clock(t);
                e.execute( ctx);
            }
            _ => {
                println!("{:?} Finished All Events",ctx.get_simulator().get_clock());

                break;
            }
        }
    }
    (ctx.get_generator().count as i64,
     ctx.get_servers().total_customers_servered as i64,
     ctx.get_simulator().get_clock().get_raw())
}

pub fn start() -> (i64,i64,i64) {
    let max_sim_time = Clock::from(1000);
    let capacity = 10;
    let mut simulator = &mut Simulator::new();
    let mut queue = &mut Queue::new();
    let mut server  = &mut Server::new();
    let mut generator  = &mut Generator::new ( max_sim_time);
    let mut events = &mut EventQueue::new();
    let mut ctx = Context {simulator, servers: server, generator, events,queue, _marker: PhantomData };
    ctx.get_queue().capacity = capacity;

    let initial_event= Event::new(EventType::GeneratorEvent,Clock::from(0));
    ctx.get_events().insert(initial_event);

    let stats =  do_all_events(&mut ctx);
    // todo: real drop
    drop(ctx);

    stats
}

#[test]
fn test_simulator() {

    let (send, recieved ,time) = start();
    println!("send {} , processed {}, tick {}", send, recieved ,time);
    assert!( recieved > 10);
}
