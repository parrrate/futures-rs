use futures::executor::block_on;
use futures::future::{self, BoxFuture, FutureExt};
use std::sync::mpsc;
use std::thread;

#[test]
fn lots() {
    const N: i32 = 100;

    fn do_it(input: (i32, i32)) -> BoxFuture<'static, i32> {
        let (n, x) = input;
        if n == 0 {
            future::ready(x).boxed()
        } else {
            future::ready((n - 1, x + n)).then(do_it).boxed()
        }
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| block_on(do_it((N, 0)).map(move |x| tx.send(x).unwrap())));
    assert_eq!((0..=N).sum::<i32>(), rx.recv().unwrap());
}
