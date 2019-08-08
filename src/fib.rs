use std::marker::PhantomData;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
///! In this example ,we transform the following function into non-recursive form,
///! evaluate and get result.
///!
///! F(0) = 0,
///! F(1) = 1,
///! F(n) = F(n-1) + F(n-2).
///!
///! Since this form can be viewed as a formal grammar, or represented by a simple AST, if we can
///! achive this goal,  which means, our Q framework can (almost, with little help by hand at this monment)
///! parse input and transform an AST to Queue Automaton!!!
///! translation rule:
///!
///! 1: functions/operators becomes Event Node(QoP)
///! 2: data dependicy translated into Queue Node (QoQ)
///!

///!
///! Adder is a implementation of operator + Fn (i32,i32) -> i32
///!

/// # Fib Operoator,
/// let's assume we translated above functions already, now we write F(n) as
///  F(n) = ( Adder  F(n-1)  F(n-2) )
///  which actually means
///  F(n) =  F(n-2) ; F(n-1) ; (dependency magic happen here ) Adder;
/// thus we mapped the grammar into a event queue
/// in a perfect world, it should be only
///
/// ```rust
/// #[derive(Debug,Clone,QoP]
/// struct Fib {
///  // input can be tracked from eventtype, or we can
///  // use a queue as well, since we see dependency on Fib(self link)
///   #[bind(EventType(input) = EventType(x))]
///   input:i32
/// }
///impl QoP for Fib {
///    fn execute( &mut self, ctx: &mut Context ){
///        if self.input == 0 || self.input  == 1 {
///            send!(n)
///        } else {
///            next!(FibEvent(n-2));
///            next!(FibEvent(n-1));
///            next!(AdderEvent);
///        }
///    }
///}
/// ```
/// but for now lets live with ugly code .
///
#[derive(Debug,Clone)]
pub struct Fib {
    input : i32,
    clock: Clock
}
// TODO: fix clock business
impl Clocked for Fib {
    fn get_clock(&self) -> Clock { self.clock   }
    fn set_clock(&mut self, t: Clock) { self.clock = t;}
}
impl Fib  {
    pub fn new(input:i32) -> Self {  Fib { input: input,   clock: Clock::from(0)}  }
    pub fn set_input( &mut self, i : i32) {self.input = i;}
}

impl AbstractEvent for Fib {
    fn execute( &mut self, ctx: &mut Context ){
        let n = self.input;
        if n == 0 || n == 1 {
            let q = ctx.get_queue();
            q.insert(ctx, n);

        } else {

            let events = ctx.get_events();

            // TODO: make a macro so next three lines can be merged into next!(FibEvent(n-2) )
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
/// #Adder Operator
/// Adder should look like this
/// ```rust
/// impl AbstractEvent2 for Adder {
///    fn execute( &mut self, ctx: &mut Context ){
///        if q.size() >= 2 {
///          recv!(a,b);
///          self.result = a + b;
///          send!(result);
///          next!();
///        } else {
///            defter!(AdderEvent);
///        }
///
///    }
/// ```
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

impl AbstractEvent for Adder {
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
            // TODO: Bug here, we need delay event, but it requires peek into
            // max value inside event queue.

            // println!("Defer {:?} : adder: {}", self.get_clock(), self.result);
            // let events = ctx.get_events();
            // TODO: change next three lines into delay!(AdderEvent)
            // self.clock =  self.clock + Clock::from(50);
            // let event = Event::new(EventType::AdderEvent, self.get_clock());
            // events.insert(event);
        }

    }
}
/// # queue
/// remember this is not event queue, it's a communication channel to model data dependency
/// it's purpose here is to register Adder event data is available.
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
/// # main
/// Our boilplate code to create operators and setup framework, this should be simplifed with a
/// builder pattern
///
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

/// # Q Framework Functions
/// code after here should be considered as library functions, we dont need change them
///
///
///
/// # Simulator (poorman's Arena)
///
/// Here it's just a DFA for the queue automaton
///
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
    ///
    /// the tick function
    ///
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
}




pub trait AbstractEvent: Clocked{
    // type S: AbstractSimulator;
    fn execute(&mut self, ctx: &mut Context);

}
type EventId = u64;
/// However, we still need write this, it could be done by using a a map using operators typeid
#[derive(Debug,Clone,Copy )]
pub enum EventType {
    FibEvent(i32),
    AdderEvent
}
///
/// Event objects, we use this to carry some small datapayload, it should be modeled properly later
/// on
/// Events are saved inside EventQueue
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

///
/// Ugly dispatch using enum, `fib.set_input(x)` is a quick hack, inside fib we still can query
/// payload contained in `current event`
///

impl AbstractEvent for Event {
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
///
/// # A Naive priority quene using BinaryHeap
///
/// TODO: we need add a new function allow us peak the furthers event, so we
/// can defer events
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
///
/// # Context (poorman's Arena)
///
/// Right now this version is far better than other safe implementation, we can use unsafecell, but
///  it just does exactly samething
/// with a replacement like slab or generation index, or, we can just delete this, if our language
/// is not rust.
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



///
/// System Tick
/// Please refer to Fib/Adder/Simulator to see how we synchronize clock and advance to next tick.
///
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
///
/// A trait defined on all object which have a clock
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