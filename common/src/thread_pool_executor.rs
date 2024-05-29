//! This file provides utilities for executing functions in parallel using a thread pool and gathering results across multiple threads.

use threadpool::ThreadPool;
use std::sync::mpsc::{channel, Sender};

/// Executes a function in parallel over a set of inputs using a thread pool, collecting the results.
///
/// This function initializes a thread pool based on the number of inputs and applies a given function concurrently.
/// The results of each function execution are collected into a vector and returned.
///
/// # Type Parameters
/// * `ResultType`: The type of the output that each function execution returns. Must be sendable between threads (`Send`) and have a static lifetime (`'static`).
/// * `FunctionType`: The function type to execute, which should take an argument of type `InputType` and return a value of type `ResultType`.
///   This function must be cloneable, sendable, and have a static lifetime.
/// * `InputType`: The type of the input elements. 
///
/// # Parameters
/// * `input`: A vector of type `InputType` whose length determines the number of times the function `FunctionType` is executed.
/// * `predicate`: A function of type `FunctionType` that is executed once per thread.
///
/// # Returns
/// A vector of `ResultType` containing the results from each thread's execution of the function.
pub fn thread_pool_executor<ResultType, FunctionType, InputType>(input: Vec<InputType>, predicate: FunctionType) -> Vec<ResultType>
    where ResultType: Send + 'static,
          FunctionType: FnOnce(InputType) -> ResultType + Send + 'static + Clone, 
          InputType: Send + 'static {

    let pool = ThreadPool::new(input.len());
    let (tx, rx) = channel();

    for item in input {
        let tx: Sender<ResultType> = tx.clone();
        let pred: FunctionType = predicate.clone();
        pool.execute(move || {
            let result: ResultType = pred(item); 
            tx.send(result).expect("channel will be there waiting for the pool");
        });
    }
    
    let mut results = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        results.push(rx.recv().expect("at least one thread should execute"));
    }

    results
}