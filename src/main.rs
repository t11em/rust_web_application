use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::executor::block_on;
use futures::future::FutureExt;

struct Number {
    value: i32,
}

impl Future for Number {
    type Output = i32; // Return i32 type

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::task::Poll::Ready(self.value)
    }
}

fn a1() -> impl Future<Output=i32> {
    Number { value: 1 }
}

fn plus(a: i32, b: i32) -> impl Future<Output=i32> {
    Number { value: a + b }
}


fn main() {
    println!("Hello, world!");
    println!("global: {}", std::env::var("GLOBAL").unwrap());
    println!("global: {}", std::env::var("LOCAL").unwrap());

    let ans = block_on(a1().then(|a| a1().then(move |b| plus(a, b))));
    println!("ans: {}", ans);
}
