use qframework::base::*;
use qframework::event::*;
use qframework::generator::*;
use qframework::server::*;
use qframework::queue::*;
use qframework::simulator::*;
use core::marker::PhantomData;

pub fn main()  {
    let max_sim_time = Clock::from(1000);
    let capacity = 10;
    let mut simulator = &mut Simulator::new();
    let mut queue = &mut Queue::new();

    let mut worker  = &mut Server::new();
    // the generator can be any thing have "execute()"
    let mut source  = &mut Generator::new ( max_sim_time);
    let mut events = &mut EventQueue::new();
    let mut ctx = Context {simulator, servers: worker, generator: source, events,queue, _marker: PhantomData };
    ctx.get_queue().capacity = capacity;

    let initial_event= Event::new(EventType::GeneratorEvent,Clock::from(0));
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