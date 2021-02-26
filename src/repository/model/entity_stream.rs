use std::pin::Pin;
use std::task::{Context, Poll};
use futures::stream::Stream;
use crate::prelude::*;
pub struct EntityStream<'a, E> {
    stream: Box<dyn Stream<Item=Result<E, Failure>> + Send + 'a>,
}

impl <'a, E>EntityStream<'a, E> {
    pub fn new(stream: impl Stream<Item=Result<E, Failure>> + Send + 'a) -> EntityStream<'a, E> {
        return EntityStream {
            stream: Box::new(stream),
        }
    }
}

impl <'a, E>Stream for EntityStream<'a, E> {
    type Item = Result<E, Failure>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let stream = unsafe {
            self
            .map_unchecked_mut(|stream| {
                return stream.stream.as_mut();
            })
        };
        return stream.poll_next(cx);
    }
}