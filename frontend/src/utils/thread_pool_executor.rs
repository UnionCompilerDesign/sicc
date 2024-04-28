use threadpool::ThreadPool;
use std::sync::mpsc::{channel, Sender};

/// Executes a function in parallel using a thread pool, collecting the results
pub fn thread_pool_executor<T, F, I>(input: Vec<I>, predicate: F) -> Vec<T>
    where T: Send + 'static,
          F: FnOnce() -> T + Send + 'static + Clone, 
          I: Send + 'static, {

    let pool: ThreadPool = ThreadPool::new(input.len());
    let (tx, rx) = channel();

    for _ in 0..input.len() {
        let tx: Sender<T> = tx.clone();
        let pred: F = predicate.clone();
        pool.execute(move || {
            let result: T = pred();
            tx.send(result).expect("channel will be there waiting for the pool");
        });
    }
    let mut results: Vec<T> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        results.push(rx.recv().expect("at least one thread should execute"));
    }

    results
}