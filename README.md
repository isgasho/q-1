# Q

The Q framework is based on one experimental simulator, as you can see in the initial code. Although it's a result of coffee crash and 24hours non-sleep thinking, I would give creit to it's [orginal creators](https://en.wikipedia.org/wiki/Queue_automaton).

# Formal Definnation

A queue automaton can be defined as a six-tuple
- M = ( Q , Σ , Γ , $ , s , δ ) where
- Q  is a finite set of states;
- Σ ⊂ Γ is the finite set of the input alphabet;
- Γ is the finite queue alphabet;
- $ ∈ Γ − Σ is the initial queue symbol;
- s ∈ Q  is the start state;
- δ : Q × Γ → Q × Γ ∗  is the transition function.

but the most interesting thing is, it's Turing complete!

# Theory

we can map queue automaton into a system which consists of :

- A event based Descrete Event Simulator
- A queue network

Queue network is a very powerful tool itself, to model many system including network, webservice, computer architecture. A event based Descrete Event Simulator can "execute" such model, with little cost.

Combine them togethoer we get a very powerful tool, which is suitable for almost all areas.

# Demo 1 : `demo.rs`
the `demo.rs` is a simple D/D/1 simulator, it can be adjust to simulator any M/G/1 or whatever network model, but here we use it as scheduler.  The Generator can be think of event source, and server can be think of event sink.

```log
$cargo run
   Compiling qframework v0.1.0 (/Users/mark/Project/q)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/demo`
@0000000000 sending  [customor 0000] 	total tx 0
@0000000001 serving  [customor 0000] 	total rx 1
@0000000001 sending  [customor 0001] 	total tx 1
@0000000002 serving  [customor 0001] 	total rx 2
@0000000002 sending  [customor 0002] 	total tx 2
@0000000003 serving  [customor 0002] 	total rx 3
@0000000003 sending  [customor 0003] 	total tx 3
@0000000004 serving  [customor 0003] 	total rx 4
@0000000004 sending  [customor 0004] 	total tx 4
@0000000005 serving  [customor 0004] 	total rx 5
@0000000005 Finished All Events
Send 5, processed 5, tick 5
```
server are allowed to drop request if it's busy, this can be easily verified by advance 2 clock on server code:
```
@0000000021 dropped  [customor 0021]
@0000000022 serving  [customor 0019] 	total rx 11
@0000000022 sending  [customor 0022] 	total tx 22
@0000000023 sending  [customor 0023] 	total tx 23
@0000000023 dropped  [customor 0023]
@0000000024 serving  [customor 0020] 	total rx 12
@0000000024 sending  [customor 0024] 	total tx 24
@0000000025 sending  [customor 0025] 	total tx 25
```
# Demo 2 : `fib.rs`
it will be a shame for such a powerful tool can only process web requests or simulate a queue network. As since it's Turing Compelete, we can do more advanced things.
How about tail call optimization and recursion, and lazy evaluation? Something Missing in rust yet!

Yes.

let's get back to old fib(n) function, see how to impement it on q framework!

first we define a thing, called Fib, and another one called Add

Fib(n) = Add ( Fib(n -1 ) , Fib (n-2), Fib(0) = 0, Fib(1) = 1.

1: Start - Add Event::Fib(8) in to event queue
2: System started, pick first event, run fib.execute()
3: inside fib.execute(), it simply add 3 events into list: Fib(6),Fib(7),Add, now Fib(8) event gone, remember we dont copy Fib, just pass a Event Type Fib(8).
4: system get called after fib.execute(), it pick the nearest future event from the priority queue(sorted already),and it's Fib(6)
5: Fib(6) add 3 event to the list, now it becomes  Fib(7),Add,Fib(5),Fib(4)
6: try run Add but we find its queue( only when we find two or more numbers in it's buffer we run addition) is empty, we push it back to end of event queue
7: repeating, eventually we have Fib(0), Fib(1),Add, on Fib(0) and Fib(1), we write number to Add's queue
8: eventually Add will get executed, it pop up two number, add them and push  result back to it's queue
9: eventually we have last two numbers on Add's queue, and we added them up, write to Add's queue again.
10: no more event in system, and we got result from Add's queue.

You can do this process on paper by hand, and you will find out we dont need maintain a deep call stack, and it's indeed lazy!

Where is the Code? `wip`.

PR welcome!

# Future Plan

For now this project is just at basic shape, ideas welcome.

- define API
- add more examples
- add 0 external dependency

# License

MIT/Apache-2.0

# Code of Conduct

stop downvote me on reddit!

# FAQ

- Is this just another name for actor in whatever OTP?
    - I never heard of it, I never use actor before, I think it's a obviouse way to write complex dynamic graph flow in this way, Since in my previouse life I write verilog and use VCS/Modelsim, and I know DSE is powerful!
    - Ask them provide formal model for actor and prove it's Turing Complete.
- Why this is called a framework? what problem it try to solve?
   - to save people from ECS/Redux, ask these developers, prove their design pattern is Turing Complete.
   - It could be a Q language impletented with macros, but it's too complicated for me.





