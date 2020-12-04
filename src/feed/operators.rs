use super::{BasicStream, Stream};

impl<'a, S> Stream<'a, S>
where
    S: Clone,
{
    pub fn apply<F>(stream: Stream<'a, S>, map_fn: F) -> Stream<S>
    where
        F: Fn(S) -> S + 'a,
        S: Clone,
    {
        let name = stream.name();
        let mut new_stream = Stream::source(stream.map(map_fn));

        new_stream.rename(name);

        new_stream
    }
}

