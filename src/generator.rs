#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cmp::Ordering;
use super::event::AbstractEvent;
use super::base::*;
use super::*;


#[derive(Debug, Clone)]
pub struct Generator {
    max_sim_time: Clock,
    clock: Clock,
    pub count: u64,

}

impl AbstractEvent for Generator {
    fn execute(&mut self, ctx: &mut Context) {
        let customer = Customer::new(self.count);
        let mut queue = ctx.get_queue();
        println!("{:?} sending  {:?} \ttotal tx {:?}", self.get_clock(), customer, self.count);
        queue.insert(ctx, customer);

        self.count += 1;
        self.clock = Clock::from(1) + self.clock;
        if self.get_clock() < self.max_sim_time {
            let event = Event::new(EventType::GeneratorEvent, self.get_clock());
            ctx.get_events().insert(event);
        }
    }

}
impl Clocked for Generator {
    fn get_clock(&self) -> Clock {
        self.clock
    }
    fn set_clock(&mut self, t: Clock) {
        self.clock = t;
    }
}

impl Generator {
    pub fn new( max_sim_time: Clock ) -> Self {
        Generator {   max_sim_time, clock: Clock::from(0), count: 0 }
    }
}

#[test]
fn test_generator() {
     let gen = Generator::new(  Clock::from(2));
     assert_eq!(gen.get_clock(), Clock::from(0));
}