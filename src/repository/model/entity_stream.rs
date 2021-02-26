use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{StreamExt, stream::Stream};
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

    pub async fn to_vec(&mut self) -> Result<Vec<E>, Failure> {
        let mut result = Vec::new();
        while let Some(item) = self.next().await {
            let item = item?;
            result.push(item);
        }
        return Ok(result);
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