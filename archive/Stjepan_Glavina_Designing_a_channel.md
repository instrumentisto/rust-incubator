---
layout: post
title:  "Designing a channel"
---

In this post, I'm going to take a critical look at channels in Rust's standard library
and present multiple issues I have with it.

Before starting, I should note that there are good reasons why the channels are
designed the way they are. Some of them boil down to the minimalistic philosophy of
the standard library, some have to do with Rust's history, and some are simply
"nobody has done it better yet".

Finally, I'll try to rethink the channel API and design one that is cleaner and more
flexible. That will be a channel that might or might not be a good fit for the standard
library, but in any case would certainly work well as an addition to Crossbeam.

How to actually implement that API will be a topic for another post.

# A look at Go's channels

Recently I've been fiddling with Go and digging into the internals of its channel implementation.
Channels are the most important feature of Go - in fact, there isn't much to Go other than
channels and goroutines!

To illustrate what its channels can do, consider the following function:

```go
func Match(name string, c chan string) {
    select {
    case peer := <-c:
        fmt.Printf("%s received a message from %s.\n", name, peer)
    case c <- name:
        // Wait for someone to receive my message.
    }
}
```

Here we select over two operations:

1. Receive a message from channel `c`.
2. Send `name` through channel `c`.

The program blocks until one of the operations succeeds. It's also important to say that
*exactly one* operation will succeed. In particular, it is not possible for these two
operations to pair up - the first case cannot receive `name` from the second case.

Selection allows for very flexible and composable synchronization.
I think Go's channels are great, and in a way, they're definitely an ideal for
channels in other languages to strive towards.

One of the most common criticisms, however, is that Go's channels are slow.
And, indeed, they are not terribly performant. It's extremely difficult to implement a
channel that is any faster but still just as powerful.

All that said, this post doesn't really aim to compare Go and Rust. I'm intentionally
pointing out the good parts of Go's channels only, but they do come with a series of their
own warts, which I won't be analyzing here.

With that in mind, let's now turn to Rust...

# Problems with Rust's channels

Rust has channels in the standard library within the `std::sync::mpsc` module.

Although the rest of the post will be very critical, my opinion is that they're generally
well designed. For example, I quite like how disconnection works: as soon as all senders
or all receivers get dropped, the channel is disconnected. Neat.

But... Rust's channels also come with several oddities. Here's one:

**Problem #1:** There is no `send_timeout` method.

`Receiver` has a blocking method `recv_timeout` that simply times out after a specified period
instead of blocking forever. It would make perfect sense for `SyncSender` to have a similarly
named method `send_timeout`, blocking while the channel is full.

But this method doesn't exist. Why? I honestly don't know.

**Problem #2:** `Sender` should implement `Sync`, but doesn't.

A rather common annoyance with channels is that senders can't be simply shared by reference
among multiple threads. They have to be cloned instead.

But that is not always good enough. For example, in [hyper](https://github.com/hyperium/hyper)
(a HTTP library) the following is a common idiom:

```rust
let (tx, rx) = channel();
Server::http(addr).listen(move |req, res| {
    tx.send("request!"); // tx does not implement Sync
});
```

This doesn't compile because `tx` cannot be shared among multiple threads. The obvious,
but also a slow and unpleasant solution would be:

```rust
let (tx, rx) = channel();
let tx = Mutex::new(tx);
Server::http(addr).listen(move |req, res| {
    tx.lock().unwrap().send("request!"); 
});
```

If `Sender` implemented `Sync`, however, this problem would be trivially solved.

In nightly Rust (as of [PR #42397](https://github.com/rust-lang/rust/pull/42397)) that
snippet would work if we used `sync_channel` instead of `channel`.
So the good news is that `SyncSender` now implements `Sync`.

But why doesn't `Sender` implement `Sync` as well? Alex Crichton, one of the main authors of Rust's
channels, [states](https://github.com/rust-lang/rust/pull/42397#issuecomment-315867774):

* *It's a real bummer that Sender isn't Sync. It should be basically!*

So yes, `Sender` should implement `Sync`, but it doesn't, and the reason is that it's very
difficult to make the current implementation thread-safe. That's just unfortunate, but it
is what it is.

**Problem #3:** Bounded channels are slow.

Channels in Rust are often praised for good performance. That's part true, part false.

Unbounded channels are great - they're performant and highly concurrent. The implementation
keeps track of the number of `Senders` and counts the number of messages sent, and then
uses one of the three variants, dynamically switching between them as needed:

1. Oneshot variant: when the sender wasn't cloned, and at most one message has been sent.
2. Single-sender variant: when the sender wasn't cloned, but at least two messages have been sent.
3. Multi-sender variant: when the sender was cloned.

This means that the channel dynamically adapts to different workloads by
switching the underlying structure. Moreover, these implementations use
*mostly-lock-free*[^1] algorithms, which makes them pretty fast. Fantastic!

But this optimization is not a free lunch: dynamic switching between variants is exactly what prevents
`Sender` from implementing `Sync`. It's very difficult to make all this machinery thread-safe.
In other words, the tradeoff is between performance and ergonomics, and in this case,
we get performance.

Bounded channels, on the other hand, are essentially just a `Mutex<VecDeque<T>>` and a `Condvar`.
In my benchmarks, they are even slower than Go's channels, which are often criticized
for bad performance. Bounded channels don't come with any cool optimization tricks.

A bit of trivia: Rust used to have fast *mostly-lock-free*[^1] bounded queues in the standard library in the
pre-1.0 era. But those were just primitive queues, not channels. The difficult part is
how to add channel-like features to a queue (blocking operations, selection, etc.).

To summarize: unbounded channels are fast, and bounded channels are slow.

[^1]: I'm using the syntagm *mostly-lock-free* to characterize concurrent queues whose majority of operations are indeed *lock-free*, but some operations might still be blocking. This is in contrast to naive mutex-based queues, e.g. `Mutex<VecDeque<T>>`.

**Problem #4:** `Sender` and `SyncSender` are distinct types, but they don't have to be.

Channels can be constructed using the following two functions:

```rust
fn channel<T>(cap: usize) -> (Sender<T>, Receiver<T>);
fn sync_channel<T>(cap: usize) -> (SyncSender<T>, Receiver<T>);
```

It's a bit peculiar how the return types share `Receiver`, but at the same time,
`Sender` and `SyncSender` are split into two distinct types. Why is that so?

I don't know the answer to that question since the split is completely unnecessary.
Both kinds of senders could be conflated to the same type without any problems. 

In fact, this works very well for the [`chan`](http://docs.rs/chan) crate.
One might argue that `Sender` and `SyncSender` are different enough to deserve the split
(one is asynchronous and the other is synchronous), but I'm not convinced.

I think the price of lost ergonomics is higher than whatever gains we might get from
having the split.

**Problem #5:** Selection doesn't support send operations.

Rust has the unstable `select!` macro, but it can only select among multiple receive
operations. Sends are not supported. This fact alone already makes channels comparatively much less
powerful than Go's channels.

The example presented in the first part simply isn't possible to express using Rust's channels.

**Problem #6:** `select!` is informally deprecated.

In [issue #12902](https://github.com/rust-lang/rust/issues/12902) a wart in the `select!` macro
was reported, but the issue was closed and won't be fixed. Alex finishes the discussion with:

* *The `select!` macro is sort of defacto deprecated nowadays in the sense that "selection over
  a number of events" is best done with the `futures` crate outside the standard library.*

**Problem #7:** Dynamic selection is unsafe and unergonomic.

There's another way of selecting over multiple channel operations: by using the `Select` struct.
This is more powerful than `select!` because it allows us to arbitrarily choose the set of
cases during runtime, rather than enumerating a fixed set of cases in a macro.

In other words, `Select` is dynamic, while `select!` is static.

The main problem with `Select` is that it's not pretty to use - just look at this
*simplified* snippet from Servo:

```rust
let mut event = {
    let sel = Select::new();
    let mut script_port = sel.handle(&self.port);
    let mut control_port = sel.handle(&self.control_port);
    unsafe {
        script_port.add();
        control_port.add();
    }
    let ret = sel.wait();
    if ret == script_port.id() {
        FromScript(self.port.recv().unwrap())
    } else if ret == control_port.id() {
        FromConstellation(self.control_port.recv().unwrap())
    } else {
        panic!("unexpected select result")
    }
};
```

The API is even unsafe! And due to all the repetition, it's easy to accidentally get it wrong.

I get the impression that channel selection in Rust is stuck in the wrong direction.
It does work... mostly, but also doesn't seem like the right solution. We can probably do better
than that.

Selection almost certainly won't be stabilized and is likely to be deprecated. Perhaps even removed?

Commonly suggested alternatives that do support selection are
[chan](https://github.com/BurntSushi/chan) and [Tokio](https://tokio.rs).
The problem with chan is that it's very inefficient - performance is explicitly a non-goal.
And Tokio, while a good solution, is simply not a drop-in replacement.

# The ideal channel

I've been thinking about the design and implementation of channels for a long time, refusing
to accept the compromises it has made. There must be a better way to design and implement channels!

So let's take a step back, start from scratch, and ask the following question:
What would our ideal channel look like?
For the moment, let's be a bit optimistic and assume that implementation issues are not
of much concern.

I'd like to present an API for my "perfect" channel. It probably won't be a good
fit for the standard library to replace `std::sync::mpsc`, but might be a nice addition to
Crossbeam.

First of all, I don't like the *synchronous/asynchronous* terminology, and would much prefer
*bounded/unbounded*, which I think feels less confusing. Secondly, I think `Sender` and
`SyncSender` should really be the same type.

Let's start with channel construction, which might look like this:

```rust
fn unbounded<T>() -> (Sender<T>, Receiver<T>);
fn bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>);
```

So far so good.

Next up is the interface for `Sender` and `Receiver`:

```rust
struct Sender<T> { ... }
struct Receiver<T> { ... }

impl<T> Sender<T> {
    fn try_send(&self, value: T) -> Result<(), TrySendError<T>>;
    fn send(&self, value: T) -> Result<(), SendError<T>>;
    fn send_timeout(&self, value: T, dur: Duration) -> Result<(), SendTimeoutError<T>>;
}

impl<T> Receiver<T> {
    fn try_recv(&self) -> Result<T, TryRecvError>;
    fn recv(&self) -> Result<T, RecvError>;
    fn recv_timeout(&self, dur: Duration) -> Result<T, RecvTimeoutError>;
}

impl<T> Clone for Sender<T> { ... }
impl<T> Clone for Receiver<T> { ... }

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}

unsafe impl<T: Send> Send for Receiver<T> {}
unsafe impl<T: Send> Sync for Receiver<T> {}
```

Note how symmetrical and simple `Sender` and `Receiver` are: both have a triple of
`try_X / X / X_timeout` methods.

Moreover, both `Sender` and `Receiver` are cloneable and implement `Send` and `Sync`. This
makes the channel an MPMC (multi-producer multi-consumer).

Sharing channels among multiple threads is very easy and unintrusive. If you want to clone
senders and receivers to share - that's okay, and if you want to share by
reference - that's also okay. Your choice!

Obviously, now that we have multiple receivers, this means that we'll have to use Crossbeam
to take care of concurrent memory deallocation.

Finally, performance is paramount: wrapping the guts of a channel in a mutex is a non-starter.
There are ways to implement concurrent queues using atomic operations, and we shouldn't accept
anything less than that. Preferably, I'd like the unbounded variant to be as fast as
`std::sync::mpsc::channel` and bounded variant to be much faster than `std::sync::mpsc::sync_channel`.

# The ideal select

Regarding selection, I'm going to immediately impose three difficult constraints:

1. Both send and receive operations must be supported.
2. Scratch hacky macros - selection must always be dynamic.
3. The API must be safe.

I've spent a long time pondering what would an API satisfying these constraints even
look like, let alone how would one go about implementing it. And in the end this is what I
came up with:

```rust
impl<T> Sender<T> {
    fn select(&self, value: T) -> Result<(), T>;
}

impl<T> Receiver<T> {
    fn select(&self) -> Result<T, ()>;
}
```

Both ends of the channel come with `select` methods. They are in fact very
similar to `try_send` and `try_recv`.

To show what exactly these methods do, let's jump straight to reimplementing the `Match` function
from the introductory Go example.

```rust
fn match(mut name: String, tx: &Sender<String>, rx: &Receiver<String>) {
    loop {
        if let Ok(peer) = rx.select() {
            println!("{} received a message from {}.", name, peer);
            break;
        }

        if let Err(s) = tx.select(name) {
            name = s;
        } else {
            // Wait for someone to receive my message.
            break;
        }
    }
}
```

Ok, what's going on here? Why do we have a loop here?

It's best to think of `select` methods as magical equivalents to `try_send` and `try_recv`.
We don't have to explicitly mark the beginning of selection - that is automatic.
Likewise, as soon as one `select` succeeds, the selection is done - we don't have to explicitly
do anything except breaking from the loop. The `select` methods might from time
to time even decide to block the current thread.

The idea is that we simply execute the same set of selection cases in a loop and break as
soon as one of them succeeds. That's all there is to it!

Great, but how would that really work?

The `select` methods access a hidden thread-local state machine to track the current state of
selection. So selection actually works as a behind-the-scenes state machine whose input are
calls to `select` methods.

In the first iteration of the loop, the state machine remembers the first case and
counts how many cases there are. All cases return errors.

Then, in the second iteration, it goes around again and attempts to fire each case, essentially
by calling `try_recv` and `try_send`.

If all tries fail, then in the third iteration all cases return errors again, but cases
get registered along the way into the state machine's conditional variable. After completing
the last call to `select` in the third iteration, the current thread is blocked.

As soon as the thread is woken up, in the fourth iteration, all cases return errors, but channels get
unregistered from the conditional variable.

The fifth iteration is the same as the second case - we attempt to fire each case in order.
If all cases fail, we continue by registering them again.

That's it. Obviously, there must be some rules one must follow when enumerating cases in a selection:

1. All cases must appear in the same order.
2. As soon as a case succeeds, the loop must be broken.

Violating the rules would not lead to code unsafety, but might cause deadlocks, and similar kinds of 
weirdness. But I think this is not a big concern since the rules are easy to uphold.
And even if there is some chance of accidentally breaking them, I believe the flexibility
this selection mechanism would give us is totally worth it.

Finally, for additional convenience we could even support a few special selection cases, for example:

```rust
loop {
    // ...

    if select::timeout(Duration::from_ms(100)) {
        // Fires after 100 milliseconds.
        break;
    }

    if select::disconnected() {
        // Fires if all selected channels are disconnected.
        break;
    }
}
```

# Final words

That would be all! I hope this post shed some light onto some of the decisions behind the
design of `std::sync::mpsc` and showed that there's more to be desired from a channel
that what it offers.

The alternative channel and selection API I've just proposed might sound too ambitious
or too unrealistic. Coming up with an API is the easiest part, but how would one even
implement it, and at the same time also *improve* performance over `std::sync::mpsc`?

Well, that is the question I want to answer in a follow-up post.
