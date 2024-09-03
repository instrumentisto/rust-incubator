---
layout: post
title:  "Lock-free Rust: Crossbeam in 2019"
---

This is a follow-up post to [Lock-freedom without garbage collection][aaron-blog-post]
from 2015, which introduced [Crossbeam][crossbeam], a Rust library that
implements efficient lock-free data structures without relying on a tracing garbage collector.

Crossbeam has gone through a long list of improvements since then, and 
it's time to showcase where it's at today. We're aiming to provide a rich
set of tools for concurrency akin to [`java.util.concurrent`][juc] and
outdo Go channels in [features][channel-features] and [performance][channel-perf].

To see what is currently offered by the library, jump to the
[documentation][docs] for an overview.

The following tour through the history of Crossbeam contains something like 150 links.
I encourage interested readers to click on them - you may find hidden treasures
of useful resources buried in here! 💎

[crossbeam]: https://github.com/crossbeam-rs/crossbeam
[juc]: https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/package-summary.html
[channel-features]: https://docs.rs/crossbeam-channel
[docs]: https://docs.rs/crossbeam

## Contents

* [What is Crossbeam?](#what-is-crossbeam)
* [2015: Early days](#2015-early-days)
* [2017: New beginnings](#2017-new-beginnings)
* [Epochs: Rewritten from scratch](#epochs-rewritten-from-scratch)
* [Channels: Improving on mpsc and Go](#channels-improving-on-mpsc-and-go)
* [Scoped threads: Makeover](#scoped-threads-makeover)
* [AtomicCell: Like Cell, but atomic!](#atomiccell-like-cell-but-atomic)
* [Synchronization: New primitives](#synchronization-new-primitives)
* [Deque: Juggling tasks in schedulers](#deque-juggling-tasks-in-schedulers)
* [Queues: Revamped](#queues-revamped)
* [Skiplist: Just around the corner](#skiplist-just-around-the-corner)
* [Utilities: Simple and handy](#utilities-simple-and-handy)
* [Where to next?](#where-to-next)
* [Thanks](#thanks)

# What is Crossbeam?

Before we get started, it's helpful to clarify a little bit what exactly Crossbeam
is and how it relates to other libraries for concurrency and parallelism.

A common question I get is how Crossbeam differs from [Rayon][rayon] and
[Tokio][tokio]. My answer is:

* Rayon splits your data into distinct pieces, gives each piece to a thread to do
  some kind of computation on it, and finally aggregates results. Its goal is to
  distribute CPU-intensive tasks onto a thread pool.

* Tokio runs tasks which sometimes need to be paused in order to wait for
  asynchronous events. Handling tons of such tasks is no problem. Its goal is
  to distribute IO-intensive tasks onto a thread pool.

* Crossbeam is all about low-level concurrency: atomics, concurrent data structures,
  synchronization primitives. Same idea as the [`std::sync`][sync] module, but bigger.
  Its goal is to provide tools on top of which libraries like Rayon and Tokio
  can be built.

[rayon]: https://github.com/rayon-rs/rayon
[tokio]: https://github.com/tokio-rs/tokio
[sync]: https://doc.rust-lang.org/std/sync/index.html

# 2015: Early days

The same year Rust 1.0 was released, [aturon][aaron-turon] published the
blog post titled [Lock-freedom without garbage collection][aaron-blog-post],
which demonstrates that one doesn't need a language with a tracing garbage collector
to write fast lock-free programs. The secret sauce is a technique called
epoch-based garbage collection, which is much different from traditional
garbage collectors and is easily implemented as a library.

In those [early days][old-crossbeam], Crossbeam had:

* [Scoped threads][old-scoped], which can borrow variables from the parent thread.
* [Epoch-based GC][old-epochs], used for building concurrent data structures.
* MPMC queues, [`MsQueue`][old-msqueue] and [`SegQueue`][old-segqueue].
* [Chase-Lev][old-chase-lev] deque, useful in [work-stealing] schedulers.
* Lock-free stack, [`TreiberStack`][old-treiber].
* Miscellaneous utilities like [`CachePadded`][old-cache-padded],
  [`ArcCell`][old-arc-cell], and [`AtomicOption`][old-atomic-option].

Then, in 2017, aturon [declared he didn't have time][internals-post] to continue
working on the project and asked the Rust community to take it over. Many
people showed interest in contributing and that is how I got involved, too.

[aaron-turon]: https://github.com/aturon
[aaron-blog-post]: https://aturon.github.io/blog/2015/08/27/epoch
[internals-post]: https://internals.rust-lang.org/t/crossbeam-request-for-help/4933
[old-crossbeam]: https://docs.rs/crossbeam/0.3.2/crossbeam/index.html
[old-scoped]: https://docs.rs/crossbeam/0.3.2/crossbeam/fn.scope.html
[old-epochs]: https://docs.rs/crossbeam/0.3.2/crossbeam/epoch/index.html
[old-msqueue]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/struct.MsQueue.html
[old-segqueue]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/struct.SegQueue.html
[old-chase-lev]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/chase_lev/index.html
[old-treiber]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/struct.TreiberStack.html
[old-cache-padded]: https://docs.rs/crossbeam/0.3.2/crossbeam/struct.CachePadded.html
[old-arc-cell]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/struct.ArcCell.html
[old-atomic-option]: https://docs.rs/crossbeam/0.3.2/crossbeam/sync/struct.AtomicOption.html
[work-stealing]: https://en.wikipedia.org/wiki/Work_stealing

# 2017: New beginnings

At that time, we discovered some pieces of Crossbeam like `AtomicOption`,
epoch-based GC, and scoped threads had soundness holes. All of them were easy to
fix but difficult to spot. Low-level concurrency is notoriously tricky and scary,
so we first made sure those bugs are ironed out before growing the library.

Organizational changes were put in place. We split the library into multiple subcrates:

* [`crossbeam-epoch`][c-epoch] - Epoch-based garbage collection.
* [`crossbeam-deque`][c-deque] - Chase-Lev deque.
* [`crossbeam-utils`][c-utils] - Scoped threads and other utilities.

The main `crossbeam` crate re-exported those subcrates. We didn't want to split crates
any further so MPMC queues and the lock-free stack were kept in the main crate for
the time being.

Next, we created the [RFCs repository][rfcs] and begun discussing the overall
direction of the project and new features we should implement. A [wiki][wiki] page
with learning resources was set up.

The first RFC we accepted was a [roadmap][roadmap] for the next 3-6 months. In
hindsight, that was overly optimistic and should've been a plan for the year...

[rfcs]: https://github.com/crossbeam-rs/rfcs
[wiki]: https://github.com/crossbeam-rs/rfcs/wiki
[roadmap]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-04-30-roadmap.md
[c-epoch]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-epoch
[c-deque]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-deque
[c-utils]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-utils

# Epochs: Rewritten from scratch

The [epoch-based GC][epoch-gc] went through a complete rewrite. The
[atomics API][atomics-api] was first revamped - soundness holes got fixed, pointer
tagging was added, more efficient atomic operations were introduced.

Next, we [redesigned][epochs-redesign] the core epoch mechanism. Pinning got more
than two times faster, garbage collection became incremental in order to reduce
pauses, manual garbage flushing was added, and we made it possible to run arbitrary
destructors before freeing memory.

Correctness of memory orderings in the garbage collector was proven in an
incredible [RFC][epochs-proof]
written by [jeehoonkang][jeehoonkang]. The proof got us more
confidence in the implementation and was a big step forward in the maturity of the
project.

He followed up with another [RFC][collector-api] that made it possible to create 
independent GC instances and use `crossbeam-epoch` in `#[no_std]`
environments. Memory allocator [elfmalloc][elfmalloc] was then able to use it
as a dependency.

Another milestone was when we realized [guards][guards] can be safely used for
pinning, so the ugly pin scopes got removed and pinning became more ergonomic.

[atomics-api]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-05-02-atomic-api.md
[epochs-redesign]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-05-23-epoch-gc.md
[jeehoonkang]: https://github.com/jeehoonkang
[epochs-proof]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-07-23-relaxed-memory.md
[collector-api]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-10-06-collector-api.md
[elfmalloc]: https://github.com/ezrosent/allocators-rs/tree/master/elfmalloc
[guards]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-11-02-guards.md
[epoch-gc]: https://docs.rs/crossbeam/0.7.1/crossbeam/epoch/index.html

# Channels: Improving on mpsc and Go

In the spring of 2017, I got interested in [channels][chan-link] because they've become the
bread-and-butter synchronization tool in Go, while our channel implementations
were lacking in many regards. My observations were:

* [`std::sync::mpsc`][mpsc] has a number of flaws. [`Sender`][sender] is not `Sync` and
  [`Receiver`][receiver] cannot even be cloned.
  [Bounded channels][sync_channel] are just deques inside mutexes and
  therefore slow. The [`select!`][old-select] macro isn't moving towards stabilization
  due to insurmountable obstacles in its design. And there are known long-standing bugs.

* Like our bounded `mpsc`, Go channels are protected by big mutexes.
  A promising [design proposal][channels-on-steroids] for lock-free channels
  was published in 2014 and there's even a [pull request][golang-pr] for it, but it has
  been open for years with little progress.

* The only way to use channels with select on stable Rust was using
  [BurntSushi][BurntSushi]'s [chan][chan-crate] crate. It was great but never
  designed for high performance and was even slower than Go channels, which is unfortunate.

My goal was to build channels that have cloneable and shareable senders and
receivers, are faster than both Go channels and `mpsc`, have `select!`, support
dynamic selection, and fix ergonomic warts in `mpsc`. I had serious doubts that
such a thing is even possible, but it was worth giving a shot to see how far we can go. 

After seven months of research and experimentation, I published [version 0.1][ver-0.1] of
[`crossbeam-channel`][crossbeam-channel], which delivered
on most of the envisioned goals. A [blog post][designing-a-channel], an
[RFC][channel-rfc], and [benchmarks][channel-perf] accompanied the release.
But it wasn't a wild success as much as I hoped. For example, some of the
responses were:

> I still can’t get over the select design.  I know it’s performant, and every
> other part of the library is great, but the conditions seems way too easy to mess up

> I also don’t like the global state with the threadlocal variable, why can’t it be
> a struct with methods?

> Not only it is brittle, but I must admit that even after reading the
> description of the mechanism several times, I am not quite clear on exactly how it works.

Back to the drawing board. After seven more months crafting a
[complicated][select-code] `select!` macro, [version 0.2][ver-0.2] was released, which
also simplified the API surface, basing decisions on the
received feedback and conclusions of a lengthy [discussion][lessons-learned] in the
issue tracker.

The release was a significant improvement, but still not perfect.
It was [not possible][dyn-sel-issue] to do dynamic selection and users complained
about dropped receivers [not closing][no-close-issue] channels, which sparked
another interesting [discussion][new-discussion], where we dug deeper into the
design space of channels and reverted some decisions.

It took five months to figure out what to do next and rewrite the selection
mechanism from scratch.
Finally, all pieces somehow [fell into the right places][fell-places] and,
with [version 0.3][ver-0.3], my dream came true - Crossbeam
channels now offer everything one could ask for, and I haven't received 
any major complaints or feature requests so far. They are fast, MPMC, and
have a powerful select.
This year might be a good time to publish version 1.0 since I don't expect API
changes anymore.

Servo [switched][servo-pr] from `mpsc` to `crossbeam-channel`, which removed
a bunch of unsafe code and the dependence on the unstable [`mpsc_select`][mpsc_select] feature.
That was an important milestone because it proved `crossbeam-channel` is mature and
reliable enough for such a big project.

I'm also intending to write an RFC with a proposal to give `mpsc` a desperately needed
refresh: improve performance, enrich the interface, and fix bugs. There's still
a [known bug][known-bug] in it, which is even [documented][doc-bug] in the standard library!
This is not a good look and we should do something about it as soon as possible.
One option is to take a subset of `crossbeam-channel` to replace the whole
guts of `mpsc`, just like we're replacing mutexes and hash tables with
[`parking_lot`][parking_lot] and [`hashbrown`][hashbrown]. But more on that another time...

Fortunately, `crossbeam-channel` has had very few bugs during its life, and none
have ever been reported by users, thanks to the [extensive suite][test-suite] of 400
tests, some of which are borrowed from `mpsc` and Go's channels.

This is just a summary of how these channels came to be. Given how much interesting
research went into producing this crate, it deserves a blog post of
its own so I hope to write more sometime!

[select-code]: https://docs.rs/crossbeam-channel/0.2.6/src/crossbeam_channel/internal/select.rs.html#889-1911
[parking_lot]: https://github.com/rust-lang/rust/pull/56410
[hashbrown]: https://github.com/rust-lang/rust/pull/56241
[mpsc]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[sender]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
[receiver]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
[sync_channel]: https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html
[old-select]: https://github.com/rust-lang/rust/issues/27800
[channels-on-steroids]: https://docs.google.com/document/d/1yIAYmbvL3JxOKOjuCyon7JhW4cSv1wy5hC0ApeGMV9s/pub
[golang-pr]: https://codereview.appspot.com/12544043
[BurntSushi]: https://github.com/BurntSushi
[chan-crate]: https://github.com/BurntSushi/chan
[designing-a-channel]: Stjepan_Glavina_Designing_a_channel.md
[channel-rfc]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-11-09-channel.md
[dyn-sel-issue]: https://github.com/crossbeam-rs/crossbeam-channel/issues/63
[no-close-issue]: https://github.com/crossbeam-rs/crossbeam-channel/issues/55
[lessons-learned]: https://github.com/crossbeam-rs/crossbeam-channel/issues/39
[new-discussion]: https://github.com/crossbeam-rs/crossbeam-channel/issues/61
[channel-perf]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel/benchmarks#results
[fell-places]: https://github.com/crossbeam-rs/crossbeam-channel/pull/106
[servo-pr]: https://github.com/servo/servo/pull/21325
[mpsc_select]: https://github.com/rust-lang/rust/issues/27800
[crossbeam-channel]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel 
[ver-0.1]: https://docs.rs/crossbeam-channel/0.1
[ver-0.2]: https://docs.rs/crossbeam-channel/0.2
[ver-0.3]: https://docs.rs/crossbeam-channel/0.3
[test-suite]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel/tests
[known-bug]: https://github.com/rust-lang/rust/issues/39364
[doc-bug]: https://doc.rust-lang.org/1.31.0/std/sync/mpsc/struct.Receiver.html#method.recv_timeout
[chan-link]: https://docs.rs/crossbeam/0.7.1/crossbeam/channel/index.html

# Scoped threads: Makeover

[Scoped threads][scoped-threads] are such a simple convenience, yet may be the feature Crossbeam
is best known for. In the past year, we fixed [soundness issues][soundness-issue]
and [bugs][scope-bugs], removed [some][remove-defer] [cruft][remove-unchecked],
polished up [thread][join-api] [joining][join-simplify], and added support for
[nested spawns][nested-spawn]. It's really just a bunch of small incremental
changes.

Some notable breaking changes were:

* In [`scope.spawn(f)`][scope-spawn], closure `f` now accepts a single argument
  of type [`&Scope`][scope-struct], which can be used for spawning nested scoped threads.
  [Rayon's scopes][rayon-spawn] use a similar pattern.

* Function [`crossbeam::scope()`][new-scope] now returns a `Result` that contains an error
  if any automatically joined child thread has panicked. Before the change,
  such panics would get silently ignored.

[soundness-issue]: https://github.com/crossbeam-rs/crossbeam-utils/pull/36
[scope-bugs]: https://github.com/crossbeam-rs/crossbeam-utils/pull/28
[remove-defer]: https://github.com/crossbeam-rs/crossbeam-utils/pull/33
[remove-unchecked]: https://github.com/crossbeam-rs/crossbeam-utils/pull/44
[join-api]: https://github.com/crossbeam-rs/crossbeam-utils/pull/7
[join-simplify]: https://github.com/crossbeam-rs/crossbeam-utils/pull/42
[nested-spawn]: https://github.com/crossbeam-rs/crossbeam-utils/pull/47
[rayon-spawn]: https://docs.rs/rayon/1.0.3/rayon/struct.Scope.html#method.spawn
[scope-spawn]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/struct.Scope.html#method.spawn
[new-scope]: https://docs.rs/crossbeam/0.7.1/crossbeam/fn.scope.html
[scope-struct]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/struct.Scope.html
[scoped-threads]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/index.html

# AtomicCell: Like Cell, but atomic!

The set of atomics provided by the [`std::sync::atomic`][std-atomic] module is not
particularly easy to use. In many ways, it feels very low-level:

* There's only a restricted set of primitive atomic types like [`AtomicBool`][atomic-bool] and
  [`AtomicUsize`][atomic-usize]. But what if we want arbitrary types to be atomic?

* [`AtomicPtr<T>`][atomic-ptr] can load and store raw pointers only, forcing us to
  use `unsafe`. It would be nice to have atomic `Box<T>` and
  `Arc<T>` instead.

* Every atomic operation needs a memory ordering. Reckless use of [`Relaxed`][relaxed]
  resulting in UB is worryingly common. Even experienced programmers
  sometimes fall into this trap.

Enter [`AtomicCell<T>`][atomic-cell], which works just like [`Cell<T>`][cell], except it can also
be shared among threads. Arbitrary types may be used with `AtomicCell<T>`,
although some operations only work with `Copy` types. Sane defaults are used
for memory orderings so one doesn't have to worry about them.

Of course, there must be some magic enabling `AtomicCell<T>` to work
with arbitrary types:

* When type `T` can be [transmuted][transmute] into `AtomicUsize`, we internally
  pretend that `T` is actually `AtomicUsize` and perform atomic operations
  that way.

* When `T` cannot be transmuted into `AtomicUsize`, a hidden global array
  of [spinlocks][spinlock] is used. To perform an atomic operation, we take
  the pointer address of the `AtomicCell<T>` and pick one of the spinlocks
  based on it. Then we lock it, pretend the `AtomicCell<T>` is just a
  `Cell<T>`, and perform the desired operation.

Most implementations of [`std::atomic<T>`][cpp-atomic] in C++ use the same
trick, so this is nothing new. However, there is one thing that
sets `AtomicCell<T>` apart: [optimistic reads][optimistic].

Our spinlocks are implemented as [sequential locks][seq-locks], which means every lock has
a *stamp*, an atomic integer that gets incremented on every write operation.
Read operations load the stamp, optimistically read data, and then
check whether the stamp has changed. If not, we're done, and if it has,
we need to retry.

This way read operations don't contend with each other, meaning they are
[very fast][optimistic-benchmarks]. Neat!

[transmute]: https://doc.rust-lang.org/std/mem/fn.transmute.html
[spinlock]: https://en.wikipedia.org/wiki/Spinlock
[std-atomic]: https://doc.rust-lang.org/std/sync/atomic/index.html
[atomic-bool]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html
[atomic-usize]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html
[atomic-ptr]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
[relaxed]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.Relaxed
[cell]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[cpp-atomic]: https://en.cppreference.com/w/cpp/atomic/atomic
[optimistic]: https://github.com/crossbeam-rs/crossbeam-utils/pull/39
[seq-locks]: https://en.wikipedia.org/wiki/Seqlock
[optimistic-benchmarks]: https://github.com/crossbeam-rs/crossbeam-utils/pull/39#issue-205046570
[atomic-cell]: https://docs.rs/crossbeam/0.7.1/crossbeam/atomic/struct.AtomicCell.html

# Synchronization: New primitives

As of recently, Crossbeam also features [synchronization primitives][syn-prim]. They
are tools in the same category as [mutexes][mutex] and [conditional variables][condvar], except
a little bit more exotic.

Currently, we have the following primitives:

* [`Parker`][parker], the same mechanism behind function [`thread::park()`][thread-park], but
  extracted for implementing custom thread notification. Tokio uses
  it for parking and unparking threads in its thread pool.
  Rayon might adopt it in the future, too.

* [`ShardedLock<T>`][sharded-lock] is like [`RwLock<T>`][rwlock], except it has an array of small
  `RwLock`s called *shards*. Each thread performing a read operation
  locks a distinct shard, thus reducing contention and making reads
  faster. However, writes are slower because they need to lock all
  shards.

* [`WaitGroup`][wait-group], which allows threads to synchronize the beginning or end
  of some computation. It is inspired by Go's [`WaitGroup`][go-wait-group] from its 
  standard library.

[syn-prim]: https://docs.rs/crossbeam/0.7.1/crossbeam/sync/index.html
[thread-park]: https://doc.rust-lang.org/std/thread/fn.park.html
[rwlock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[condvar]: https://doc.rust-lang.org/std/sync/struct.Condvar.html
[go-wait-group]: https://golang.org/pkg/sync/#WaitGroup
[parker]: https://docs.rs/crossbeam/0.7.1/crossbeam/sync/struct.Parker.html
[sharded-lock]: https://docs.rs/crossbeam/0.7.1/crossbeam/sync/struct.ShardedLock.html
[wait-group]: https://docs.rs/crossbeam/0.7.1/crossbeam/sync/struct.WaitGroup.html

# Deque: Juggling tasks in schedulers

Almost every work-stealing task scheduler has the same setup:

* There is a shared global queue of tasks, usually called *injector*
  and is the entry point for new tasks. For example, if you call
  [`rayon::spawn(task)`][rayon-spawn] outside Rayon's thread pool, the task will be
  pushed into the global queue. Any worker thread is then allowed to
  take tasks from it.

* Each thread in the thread pool has its own *worker* queue. Only the
  thread that owns it is allowed to push and pop tasks, but other
  threads may *steal* tasks, which is a particular operation optimized
  for task scheduling.

* The job of each worker thread is to wait for tasks to appear and run
  them. To find the next task to run, a thread will first look
  into its worker queue. If empty, it looks into the global queue or
  attempts to steal tasks from other threads.

This setup is used in Rayon, Tokio, Go, [Thread Building Blocks][tbb], you name it.
The advantage of work stealing is automatic work balancing among all threads even
in presence of skewed workloads.

Crossbeam's [deque][deque-docs] originally started with a basic Chase-Lev for
work stealing, but it got beefed up since then:

* We [added support][fifo-and-lifo] for FIFO worker queues in addition
  to the classic LIFO queue. LIFO order makes sense for Rayon because it
  prioritizes tasks for cache utilization, while FIFO makes more
  sense for Tokio because it prioritizes tasks for fairness.

* [Batched steal][steal-many] operations were added, which significantly reduce
  the total cost of queue operations. They got us nice
  [speedups][tokio-steal-many] in Tokio.

* A special [`Injector`][injector] queue was [introduced][add-injector], which
  integrates nicely with [`Worker`][worker] queues and supports similar operations.

Every such improvement in [`crossbeam-deque`][cr-deque] has a ripple effect on the
library ecosystem. By bumping dependency versions and leveraging new
features, Tokio's thread pool [gets faster][get-fast], and therefore every
application using Tokio gets faster, too!

[deque-docs]: https://docs.rs/crossbeam/0.7.1/crossbeam/deque/index.html
[fifo-and-lifo]: https://github.com/crossbeam-rs/crossbeam-deque/pull/13
[steal-many]: https://github.com/crossbeam-rs/crossbeam-deque/pull/15
[tokio-steal-many]: https://github.com/tokio-rs/tokio/pull/534
[add-injector]: https://github.com/crossbeam-rs/crossbeam/pull/290
[rayon-spawn]: https://docs.rs/rayon/1.0.3/rayon/fn.spawn.html
[tbb]: https://oneapi-src.github.io/oneTBB/main/tbb_userguide/How_Task_Scheduler_Works.html
[injector]: https://docs.rs/crossbeam/0.7.1/crossbeam/deque/struct.Injector.html
[worker]: https://docs.rs/crossbeam/0.7.1/crossbeam/deque/struct.Worker.html
[get-fast]: https://github.com/tokio-rs/tokio/pull/874
[cr-deque]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-deque

# Queues: Revamped

Until very recently, there were just two unbounded MPMC queues in
Crossbeam:

* `MsQueue`, the classic lock-free [Michael-Scott][michael-scott] queue. It
  allocates on every push operation, putting high pressure on the
  global allocator. An interesting extra feature it has is blocking
  pop operation.

* `SegQueue`, which is like `MsQueue`, except it allocates segments of
  nodes. Even though it's not strictly speaking lock-free, fewer allocations and
  better cache locality make it quite a bit faster than `MsQueue`
  in practically every case.

Since `MsQueue` offers almost nothing over `SegQueue`, we've decided
to remove it. And if one needs blocking pop operations, channels can
be used as an alternative.

Then we added [`ArrayQueue`][array-queue], which is based on [dvyukov][dvyukov]'s
[bounded MPMC][bounded-mpmc] queue. The original implementation in C++ has two rough edges:
it is not [linearizable][linearizability] and forces the capacity to always be a power
of two. We've smoothened both.

A month ago, I had an epiphany and realized the bounded MPMC queue
can be generalized to an unbounded queue. By combining segments
from `SegQueue` and turning the *sequence* field into something
functionally akin to [hazard pointers][hazards], we can replace epoch-based GC
with a [different garbage collection scheme][diff-gc], bringing two benefits:

* Garbage collection is entirely eager rather than lazy. Under high
  concurrency, it can be measured that this new queue uses less
  memory than the old `SegQueue`.

* Epoch-based GC incurs a certain overhead on every operation due
  to thread pinning and occasional garbage collection. By removing
  it we get performance wins. 

`SegQueue` was then rewritten from scratch and got us much better
[benchmark][segqueue-pr] numbers.

Both `ArrayQueue` and the new `SegQueue` implementations
sprung from the effort of [optimizing][opt-channels] channels, which were the first
to use those queues internally, and then we just ripped them out
and moved into [`crossbeam-queue`][crossbeam-queue].

[dvyukov]: https://github.com/dvyukov
[hazards]: https://en.wikipedia.org/wiki/Hazard_pointer
[michael-scott]: http://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf
[segqueue-pr]: https://github.com/crossbeam-rs/crossbeam/pull/291
[bounded-mpmc]: http://www.1024cores.net/home/lock-free-algorithms/queues/bounded-mpmc-queue
[linearizability]: https://en.wikipedia.org/wiki/Linearizability
[opt-channels]: https://github.com/crossbeam-rs/crossbeam/pull/279#issue-241302461
[diff-gc]: https://github.com/crossbeam-rs/crossbeam/pull/279#issuecomment-450490718
[array-queue]: https://docs.rs/crossbeam/0.7.1/crossbeam/queue/struct.ArrayQueue.html
[crossbeam-queue]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-queue

# Skiplist: Just around the corner

There's an experimental crate [`crossbeam-skiplist`][crossbeam-skiplist] featuring
maps and sets based on lock-free [skiplists][skiplist]. They are very similar
to [`BTreeMap`][btreemap] and [`BTreeSet`][btreeset] in functionality and interface, but
can be mutated concurrently from multiple threads.

These maps and sets scale very well, much better than mutex-protected
[B-tree][btree] based equivalents. However, in single-threaded workloads,
skiplists offer rather underwhelming performance, generally three to four
times slower in comparison to B-trees.

The main inspiration here has been Java's [`ConcurrentSkipListMap`][cslm].
I've always been jealous of Java having concurrent skiplists that
are straightforward to use, so this is an effort to bring them to
Rust as well. The fact that Rust doesn't use the JVM to rely on a
mature and well-engineered concurrent GC made implementation
much more difficult for us. Surely one of my most complex pieces
of code!

The crate has been in a *coming soon* state for a long time now.
We've just been slacking off on writing additional tests and documentation to
push it over the finish line and publish the first version. But
really, it's been completed - you can clone the
[repository][crossbeam-skiplist] and play with it.

[cslm]: https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/ConcurrentSkipListMap.html
[skiplist]: https://en.wikipedia.org/wiki/Skip_list
[skiplist-rfc]: https://github.com/crossbeam-rs/rfcs/blob/master/text/2018-01-14-skiplist.md
[btree]: https://en.wikipedia.org/wiki/B-tree
[crossbeam-skiplist]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-skiplist
[btreemap]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[btreeset]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

# Utilities: Simple and handy

There are two [utilities][utils-docs] that are difficult to put into any
category. While very simple, they are indispensable in low-level
concurrency.

[`Backoff`][backoff] performs exponential backoff in spin loops by executing the
[*PAUSE* or *YIELD*][spin_loop_hint] instruction and [yielding][yield_now] the current thread to the
OS scheduler. Here's how one might use it to wait for an `AtomicBool`
to become `true`:

```rust
fn spin_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(SeqCst) {
        backoff.snooze();
    }
}
```

[`CachePadded<T>`][cache-padded] wraps a value of type `T`, padding it and aligning
to the length of a cache line. Useful when we want to make sure an
atomic variable doesn't get [falsely shared][false-sharing]. For example, in a MPMC
queue, it's a good idea to put the head and tail indices into their
own cache lines:

```rust
struct Queue<T> {
    head: CachePadded<AtomicUsize>,
    tail: CachePadded<AtomicUsize>,
    buffer: Buffer<T>,
}
```

[utils-docs]: https://docs.rs/crossbeam/0.7.1/crossbeam/utils/index.html
[spin_loop_hint]: https://doc.rust-lang.org/std/sync/atomic/fn.spin_loop_hint.html
[yield_now]: https://doc.rust-lang.org/std/thread/fn.yield_now.html
[false-sharing]: https://en.wikipedia.org/wiki/False_sharing
[backoff]: https://docs.rs/crossbeam/0.7.1/crossbeam/utils/struct.Backoff.html
[cache-padded]: https://docs.rs/crossbeam/0.7.1/crossbeam/utils/struct.CachePadded.html

# Where to next?

Crossbeam has come a long way already, but there is still
work to do. In summary, there are two sorely missing features
we need as soon as possible, and a list of less critical nice-to-haves.

In an interesting [blog post][hniksic-blog] comparing concurrency
in Java and Rust, [hniksic][hniksic-gh] points out we don't really
have a satisfactory equivalent of [`AtomicReference`][atomicref].
The closest one we have today is probably [`arc-swap`][arc-swap],
but there is still room for improvement - for example, in some
cases, it is an order of magnitude slower than `AtomicReference`.

Another [common request][servo-ht] is a concurrent hash table.
While we do have wonderful crates like [`evmap`][evmap], they tend
to be designed for niche uses and often have non-standard
interfaces. I'd love to also see a generic concurrent
hash table that works well enough across a broad set of
applications, with few surprises, and is ideally as similar to
[`HashMap`][hashmap] as possible.

So these are the things I'd like to see in Crossbeam this year:
[`AtomicReference`][atomicref] and [`ConcurrentHashMap`][chm] written in Rust.
There is a lot of design and
implementation work to do here, but we generally know how to move forward.
If you're interested, take a peek at discussions on
[atomic references][arccell-thread] and [hash tables][hashmap-thread]
in the issue tracker.

Finally, there is a neverending list of lower-priority features
we might want to explore, but are not essential to the project:

* A garbage collector similar to [`crossbeam-epoch`][crossbeam-epoch] in interface, but
  using [hazard pointers][hazptr] rather than epochs.

* Fast and low-latency SPSC queues. Perhaps also other kinds of
  specialized queues like SPMC and MPSC.

* [Shared handles][shared-handle] for the epoch-based GC. They would
  make it easier to build some concurrent data structures and remove the
  [`'static` bound][static-bound] from skiplists.

* [Eventcounts][eventcounts], [semaphores][semaphore], [phasers][phaser],
  [sharded counters][sharded-counters], [flat combining][flatcomb], ...

[hazptr]: https://en.wikipedia.org/wiki/Hazard_pointer
[hniksic-blog]: https://morestina.net/blog/784/exploring-lock-free-rust-3-crossbeam
[hniksic-gh]: https://github.com/hniksic
[arc-swap]: https://docs.rs/arc-swap
[chashmap]: https://docs.rs/chashmap
[evmap]: https://docs.rs/evmap
[arccell-thread]: https://github.com/crossbeam-rs/crossbeam/issues/160
[hashmap-thread]: https://github.com/crossbeam-rs/rfcs/issues/32
[shared-handle]: https://github.com/crossbeam-rs/crossbeam/issues/207
[static-bound]: https://github.com/crossbeam-rs/crossbeam/issues/205
[atomicref]: https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/atomic/AtomicReference.html
[servo-ht]: https://github.com/servo/servo/issues/22334
[hashmap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[chm]: https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/ConcurrentHashMap.html
[eventcounts]: http://www.1024cores.net/home/lock-free-algorithms/eventcounts
[semaphore]: https://en.wikipedia.org/wiki/Semaphore_(programming)
[phaser]: https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/Phaser.html
[sharded-counters]: https://github.com/golang/go/issues/18802
[flatcomb]: https://www.cs.bgu.ac.il/~hendlerd/papers/flat-combining.pdf
[crossbeam-epoch]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-epoch

# Thanks

Looking back at the past two years, I'm thrilled with what we've
accomplished so far! These days Crossbeam is even
[getting](https://people.mpi-sws.org/~dreyer/papers/rbrlx/paper.pdf)
[referenced](https://dl.acm.org/citation.cfm?id=2926707)
in
[research](https://www.microsoft.com/en-us/research/wp-content/uploads/2017/07/snowflake-extended.pdf)
[papers](https://plv.mpi-sws.org/scfix/paper.pdf).

My work on Crossbeam was sponsored by Mozilla and patrons on [Patreon][patreon],
to which I owe a big thank you! Without your support,
it just wouldn't be able to happen. Sustainability of open source work
is a complicated topic and, instead of going into a ramble,
I'll just say everything
[withoutboats][boats] wrote about it at the end of their [blog post][org-debt]
is spot on and rings true for me personally.

I'd also like to thank all contributors who write code, participate in
discussions, and share their feedback. This experience
made me realize firsthand just how much every single contribution
means, even if it's as simple as, say, a comment in the issue tracker.

On a final note, I feel humbled by technical and leadership knowledge
of so many people involved with Rust. And Crossbeam has contributors
whose grasp of wide ranges of topics is sometimes way over my head.
I've learned a lot from you and still am!

[patreon]: https://www.patreon.com/stjepang
[boats]: https://github.com/withoutboats
[org-debt]: https://boats.gitlab.io/blog/post/rust-2019
