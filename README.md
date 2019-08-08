# Q

The Q framework is an implementation of a generic [queue automaton](https://en.wikipedia.org/wiki/Queue_automaton) runtime.


# Theory

the implementation consists of :

- A event based Descrete Event Simulator(DES)
- A Queue Network, consists of Servers(Operator), Queues(Communication Channel)

Queue network is a very powerful tool itself, can be used on modeling network performance. A event based Descrete

Event Simulator can "execute" such model, with little cost.

Combine them togethoer we get a very powerful tool, which is suitable for almost all areas.

# Getting started

For now we just provide a single file example which transform following grammar into queue automaton and excuted.

Fib(n) = Add ( Fib(n -1 ) , Fib (n-2), Fib(0) = 0, Fib(1) = 1.


```
  cargo run -- 5
@0000000000 queue = []
@0000000005 queue = [1]
@0000000007 queue = [1, 1]
@0000000012 queue = [2]
@0000000013 queue = [2, 1]
@0000000015 queue = [3]
@0000000016 queue = [3, 0]
@0000000018 queue = [3]
@0000000018 queue = [3, 0]
@0000000019 queue = [3]
@0000000020 queue = [3, 1]
@0000000023 queue = [4]
@0000000023 queue = [4, 1]
@0000000024 queue = [5]
@0000000025 queue = [5, 0]
@0000000027 queue = [5]
@0000000027 queue = [5, 0]
@0000000028 queue = [5]
@0000000029 queue = [5, 0]
@0000000030 queue = [5, 0]
@0000000030 queue = [5, 0]
@0000000031 queue = [5]
Result 5
```
some event got delayed (adder need at least 2 numbers ready to process).

What does it mean?
- We implemented a iterperator for a specifc grammar(Fib Language)
- We used only 1 adder and 1 Fib instance(think of hardware component), or we are free of stack(queue is much like heap)

PR welcome!

# Future Plan

For now this project is just at basic shape, ideas welcome.

- define API
- add more examples
- add 0 external dependency

# License

MIT/Apache-2.0

# Code of Conduct

Any PR are welcome

# FAQ

- Is this just another name for actor in whatever OTP?
    - I had never used erlang or used any actor system, it's still dabatable as actor model havn't yet get a formal model
    - this implementation itself can be formulized using primtive operations
- Why this is called a framework? what problem it try to solve?
   - It's a very small but powerful piece, as you can see, we can





