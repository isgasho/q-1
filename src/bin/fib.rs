#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::marker::PhantomData;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Debug,Clone)]
pub struct Fib {
    input : i32,
    clock: Clock
}
impl Clocked for Fib {
    fn get_clock(&self) -> Clock { self.clock   }
    fn set_clock(&mut self, t: Clock) { self.clock = t;}
}
impl Fib  {
    pub fn new(input:i32) -> Self {  Fib { input: input,   clock: Clock::from(0)}  }
    pub fn set_input( &mut self, i : i32) {self.input = i;}
}

impl AbstractEvent2 for Fib {
    fn execute( &mut self, ctx: &mut Context ){
        let n = self.input;
        // if self.input != input {
        //     self.input = input ;
        // }
        // println!("Step {:?} : Fib({})", self.get_clock(), n);
        if n == 0 || n == 1 {
            let q = ctx.get_queue();
            q.insert(ctx, n);

        } else {
            let events = ctx.get_events();
            self.clock =  self.clock + Clock::from(1);
            let event =Event::new(EventType::FibEvent(n-2), self.get_clock()) ;
            events.insert(event);

            self.clock =  self.clock + Clock::from(1);
            let event =Event::new(EventType::FibEvent(n-1), self.get_clock()) ;
            events.insert(event);

            self.clock =  self.clock + Clock::from(1);
            let event =Event::new(EventType::FibEvent(n-2), self.get_clock()) ;
            events.insert(event);

        }

    }
}

#[derive(Debug,Clone)]
pub struct Adder {
    clock: Clock,
    pub result: i32
}
impl Clocked for Adder {
    fn get_clock(&self) -> Clock { self.clock   }
    fn set_clock(&mut self, t: Clock) { self.clock = t;}
}
impl Adder  {
    pub fn new() -> Self {  Adder {  clock: Clock::from(0), result: 0 }  }
}

impl AbstractEvent2 for Adder {
    fn execute( &mut self, ctx: &mut Context ){
        let q = ctx.get_queue();
        if q.size() >= 2 {
            let a = q.remove().unwrap();
            let b = q.remove().unwrap();
            self.result = a + b;
            // println!("Step {:?} : {} + {} => {}", self.get_clock(), a,b,self.result);
            q.insert(ctx, self.result);
            self.clock = self.clock + Clock::from(1);
        } else {
            // defter
            // println!("Defer {:?} : adder: {}", self.get_clock(), self.result);

            // let events = ctx.get_events();
            // self.clock =  self.clock + Clock::from(50);
            // let event = Event::new(EventType::AdderEvent, self.get_clock());
            // events.insert(event);
        }

    }
}
use std::env;
pub fn main()  {
    let args: Vec<String> = env::args().collect();
    let n :i32 = args[1].parse().unwrap();
    let max_sim_time = Clock::from(500000);
    let capacity = 1000;
    let  simulator = &mut Simulator::new();
    let  queue = &mut Queue::new();
    let  adder  = &mut Adder::new();
    // the generator can be any thing have "execute()"
    let  fib  = &mut Fib::new ( n);
    let  events = &mut EventQueue::new();
    let mut ctx = Context {simulator, fib, adder,events,queue, _marker: PhantomData };
    ctx.get_queue().capacity = capacity;
    let initial_event= Event::new(EventType::FibEvent(n),Clock::from(0));
    ctx.get_events().insert(initial_event);

    let stats =  run(&mut ctx);
    // todo: real drop
    drop(ctx);

    println!("Send {}, processed {}, tick {} ", stats.0, stats.1, stats.2 );
}

pub fn run(ctx: &mut  Context) -> (i64,i64,i64) {
    loop {
        let event =  ctx.get_events().remove_first();
        match event {
            Some(mut e) => {
                let t = e.get_clock() ;
                ctx.get_simulator().set_clock(t);
                println!("{:?} queue = {:?} ",ctx.get_simulator().get_clock() ,
                ctx.get_queue());
                // quick hack, use event type to change input
                e.execute( ctx);
            }
            _ => {
                println!("{:?} Finished All Events",ctx.get_simulator().get_clock());
                println!("Result {}", ctx.get_adder().result);
                break;
            }
        }
    }
    (ctx.get_fib().input as i64,
     ctx.get_adder().result as i64,
     ctx.get_simulator().get_clock().get_raw())
}


pub trait AbstractEvent2 : Clocked{
    // type S: AbstractSimulator;
    fn execute(&mut self, ctx: &mut Context);

}
type EventId = u64;
#[derive(Debug,Clone,Copy )]
pub enum EventType {
    FibEvent(i32),
    AdderEvent
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

impl AbstractEvent2 for Event {
    fn execute(&mut self, ctx : &mut Context) {
        // info!("Running Event");
        match self.etype {
            EventType::AdderEvent => {
                let mut adder = ctx.get_adder();
                adder.execute(ctx);
            },
            EventType::FibEvent(x) =>{
                let mut fib = ctx.get_fib();
                fib.set_input(x);
                fib.execute(ctx);
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
pub struct Context<'a , T = i32>{
    pub simulator: *mut Simulator,
    pub fib : *mut Fib,
    pub adder: *mut Adder,
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
    pub fn get_fib(&self) -> &'a mut Fib {
        let ptr =  unsafe { &mut *self.fib };
        ptr

    }
    pub fn get_adder(&self) -> &'a mut Adder {
        let ptr =  unsafe { &mut *self.adder };
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
#[derive(Debug)]
pub struct Queue{
    fibs: Vec<i32>,
    pub capacity: usize,
}
impl Queue  {
    pub fn new() -> Self {
        return Queue {fibs: Default::default(), capacity: 10};
    }
    pub fn size(&self) -> usize {
        self.fibs.len()
    }
    pub fn insert( &mut self, ctx: &mut Context, fib: i32) {
        let mut sim = ctx.get_simulator();
        if self.size() >= 2 {
            // let mut  adder = ctx.get_adder();
            // adder.execute(ctx);
            let events = ctx.get_events();
            let clock =  sim.get_clock() + Clock::from(1);
            let event = Event::new(EventType::AdderEvent, clock);
            events.insert(event);
        } else {
            self.fibs.push(fib);
        }

    }
    pub fn remove(&mut self) -> Option<i32>{
        self.fibs.pop()
    }

}
pub trait Clocked{
    fn now(&self) -> Clock {
        self.get_clock()
    }
    fn get_clock(&self) -> Clock ;
    fn set_clock(&mut self, t: Clock);
}
// impl std::ops::Add for Clock {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Self::from(self.0 + other.0)
//     }
// }

// impl std::fmt::Debug for Clock {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "@{:010.03}", self.0)
//     }
// }