
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::cmp::Ordering;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use super::*;
use super::base::*;

#[derive(Debug,Clone)]
pub struct Server {
    pub total_customers_servered :i32,
    pub customer_being_served : Option<Customer>,
    clock: Clock
}
impl Clocked for Server {
    fn get_clock(&self) -> Clock {
        self.clock
    }
    fn set_clock(&mut self, t: Clock) {
        self.clock = t;
    }
}
impl Server  {


    pub fn is_available(&self) -> bool {
        self.customer_being_served.is_none()
    }

    pub fn new() -> Self {
        Server { total_customers_servered: Default::default(),
                customer_being_served: Default::default(),
                clock: Clock::from(0)}
    }
    pub fn insert( &mut self, ctx: &mut Context, customer: Option<Customer>) {
        if self.customer_being_served.is_some() {
            return;
        } else {
            self.customer_being_served = customer;
            self.clock = self.clock + Clock::from(1);
            let event = Event::new(EventType::ServerEvent, self.get_clock());
            ctx.get_events().insert(event);
        }
    }
}

impl AbstractEvent for Server {
    fn execute( &mut self, ctx: &mut Context ){
        let c = self.customer_being_served.take();
        self.total_customers_servered += 1;
        println!("{:?} serving  {:?} \ttotal served {:?}", self.get_clock(), c, self.total_customers_servered);
        let q = ctx.get_queue();
        if let  Some(c) = q.remove() {
            self.insert(ctx, Some(c) );
        }
    }
    }


#[test]
fn test_server() {
     let serv :Server = Server::new();
     assert_eq!(serv.get_clock(), Clock::from(0));

}