use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Default)]
pub struct Oneshot(bool);

impl Future for Oneshot {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;

            cx.waker().wake_by_ref();

            Poll::Pending
        }
    }
}
