Step 1.9: Phantom types
=======================

__Estimated time__: 1 day

Because [Rust] has a rich type system, a programming logic and semantics are mostly expressed in types rather than in data/values, which is known as a "programming with types" concept. Often, this leads to situations where you need to express some type relations without having any values of those types. Here is where [phantom types][5] come in: they carry some semantics on type level, which invariants are checked by compiler, and are totally compiled out in runtime.

> A phantom type parameter is simply a type parameter which is never used.

However, in [Rust], this often causes the compiler to complain, and the solution is to add a "dummy" use by way of [`PhantomData`].

This is a quite common practice when you're writing a highly abstracted generics code. A real-world example (and somewhat scary) would be:
```rust
trait CommandGateway<C: Command> {
    type Result;
    
    fn command(&self, cmd: C) -> Self::Result;
}

// Here we need to abstract over some types
// to be able to use them in CommandGateway implementation.
pub struct Snapshotter<Repo, AggEv, Err> {
    repo: Repo,
    _aggregate_event: PhantomData<AggEv>,
    _error: PhantomData<Err>,
}

impl<Cmd, Repo, AggEv, Err> CommandGateway<Cmd>
    for Snapshotter<Repo, AggEv, Err>
where
    Cmd: Command + 'static,
    Cmd::Aggregate: VersionedAggregate
        + EventMessageSourced<
            AggEv,
            EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        >,
    Repo: AggregateRepository<Cmd::Aggregate>
        + EventStore<
            Cmd::Aggregate,
            Event = AggEv,
            EventMeta = EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        > + Clone
        + 'static,
    AggEv: AggregateEvent + 'static,
    Err: From<
            SnapshotterError<
                <Repo as AggregateRepository<Cmd::Aggregate>>::Error,
                <Repo as EventStore<Cmd::Aggregate>>::Error,
            >,
        > + 'static,
{
    type Result = DynFuture<Option<Cmd::Aggregate>, Err>;
    
    fn command(&self, cmd: Cmd) -> Self::Result {
        let repo = self.repo.clone();
        let fut = repo
            .load(cmd.aggregate_id().unwrap())
            .map_err(AggregateLoadingFailed)
            .map(|agg| agg.unwrap_or_else(Cmd::Aggregate::initial_state))
            .and_then(move |agg| {
                repo.read_events(
                    cmd.aggregate_id().unwrap(),
                    Some(agg.version()),
                )
                .map_err(EventsReadingFailed)
                .fold((agg, false), |(mut agg, _), ev| {
                    agg.apply_event_message(&ev);
                    Ok((agg, true))
                })
                .map(move |(agg, changed)| (agg, changed, repo))
            })
            .and_then(|(agg, has_changed, repo)| {
                if has_changed {
                    Either::A(
                        repo.store(&agg)
                            .map_err(AggregateStoringFailed)
                            .map(move |_| Some(agg)),
                    )
                } else {
                    Either::B(future::ok(None))
                }
            });
        Box::new(fut.map_err(Err::from))
    }
}
```

For better understanding [`PhantomData`] purpose, design, limitations and use cases, read through the following articles:
- [Official `PhantomData` docs][`PhantomData`]
- [Rust By Example: 14.9. Phantom type parameters][1]
- [Rustonomicon: 3.10. PhantomData][2]
- [Reddit: Why PhantomData][3]
- [RIP Tutorial: Using PhantomData as a Type Marker][4]




## Task

Implement a `Fact<T>` type which returns some random fact about `T` type that `Fact<T>` is implemented for.

```rust
let f: Fact<Vec<T>> = Fact::new();
println!("Fact about Vec: {}", f.fact());
println!("Fact about Vec: {}", f.fact());
```
```
Fact about Vec: Vec is heap-allocated.
Fact about Vec: Vec may re-allocate on growing.
```





[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/rust-by-example/generics/phantom.html
[2]: https://doc.rust-lang.org/nomicon/phantom-data.html
[3]: https://www.reddit.com/r/rust/comments/8oqj14/why_phantomdata
[4]: https://riptutorial.com/rust/example/24109/using-phantomdata-as-a-type-marker
[5]: https://stackoverflow.com/questions/28247543/motivation-behind-phantom-types
