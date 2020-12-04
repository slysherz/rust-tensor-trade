use super::Stream;
use crate::oms::exchanges::StreamLike;

impl<'a, S> Stream<'a, S>
where
    S: Clone,
{
    pub fn apply<F, R>(stream: Stream<'a, S>, map_fn: F) -> Stream<R>
    where
        F: Fn(S) -> R + 'a,
        S: Clone,
        R: Clone,
    {
        let name = stream.name();
        let mut new_stream = Stream::source(stream.map(map_fn));

        new_stream.rename(name);

        new_stream
    }
}
