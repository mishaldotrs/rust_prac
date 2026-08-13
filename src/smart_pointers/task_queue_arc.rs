//! A shared, thread-safe task queue built on Arc<Mutex<VecDeque<T>>> + Condvar.
//!
//! - `Arc` lets multiple threads share ownership of the queue.
//! - `Mutex` guards the underlying `VecDeque` so only one thread mutates it at a time.
//! - `Condvar` lets worker threads block efficiently instead of busy-polling when
//!   the queue is empty, and wakes them up as soon as work (or shutdown) arrives.
//!
//! Run with: `rustc task_queue.rs -O && ./task_queue`
//! Or drop the `TaskQueue` struct into your own project (no external crates needed).

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// Internal state protected by the mutex.
struct Inner<T> {
    queue: VecDeque<T>,
    shutdown: bool,
}

/// A thread-safe FIFO queue that can be cloned (via `Arc`) and shared across threads.
pub struct TaskQueue<T> {
    inner: Mutex<Inner<T>>,
    not_empty: Condvar,
}

impl<T> TaskQueue<T> {
    /// Create a new, empty queue wrapped in an `Arc` so it can be shared directly.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner {
                queue: VecDeque::new(),
                shutdown: false,
            }),
            not_empty: Condvar::new(),
        })
    }

    /// Push a task onto the back of the queue and wake one waiting consumer.
    pub fn push(&self, task: T) {
        let mut guard = self.inner.lock().unwrap();
        guard.queue.push_back(task);
        // Wake up a single waiting thread; cheaper than notify_all when only
        // one consumer needs to run per item.
        self.not_empty.notify_one();
    }

    /// Pop a task, blocking the calling thread until one is available or the
    /// queue is shut down. Returns `None` once shutdown and drained.
    pub fn pop(&self) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        loop {
            if let Some(task) = guard.queue.pop_front() {
                return Some(task);
            }
            if guard.shutdown {
                return None;
            }
            // Atomically unlocks the mutex and sleeps; re-locks on wakeup.
            // Using a loop (not `if`) guards against spurious wakeups.
            guard = self.not_empty.wait(guard).unwrap();
        }
    }

    /// Try to pop without blocking. Returns `None` if the queue is currently empty.
    pub fn try_pop(&self) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        guard.queue.pop_front()
    }

    /// Pop with a timeout; returns `None` if nothing arrives in time or the
    /// queue shuts down.
    pub fn pop_timeout(&self, timeout: Duration) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        loop {
            if let Some(task) = guard.queue.pop_front() {
                return Some(task);
            }
            if guard.shutdown {
                return None;
            }
            let (new_guard, result) = self.not_empty.wait_timeout(guard, timeout).unwrap();
            guard = new_guard;
            if result.timed_out() {
                return guard.queue.pop_front();
            }
        }
    }

    /// Signal shutdown: wakes every blocked consumer so they can exit cleanly
    /// once the queue drains. No more items should be pushed after this.
    pub fn shutdown(&self) {
        let mut guard = self.inner.lock().unwrap();
        guard.shutdown = true;
        self.not_empty.notify_all();
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ---------------------------------------------------------------------------
// Demo: a small worker pool consuming from a shared TaskQueue<u32>.
// ---------------------------------------------------------------------------
pub fn task_queue() {
    let queue = TaskQueue::<u32>::new();
    let num_workers = 4;

    // Spawn workers. Each gets its own clone of the Arc (cheap: bumps a
    // refcount, all clones point at the same TaskQueue).
    let mut handles = Vec::new();
    for id in 0..num_workers {
        let q = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            // Loop until pop() returns None (queue shut down and drained).
            while let Some(task) = q.pop() {
                println!("worker {id} processing task {task}");
                thread::sleep(Duration::from_millis(20)); // simulate work
            }
            println!("worker {id} exiting");
        }));
    }

    // Producer: push 20 tasks from the main thread.
    for task in 0..20 {
        queue.push(task);
    }

    // No more work coming: tell the queue to shut down. Workers will finish
    // draining whatever's left, then exit their loops.
    queue.shutdown();

    for h in handles {
        h.join().unwrap();
    }

    println!("all workers done, queue empty: {}", queue.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_order() {
        let q = TaskQueue::<i32>::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.try_pop(), Some(1));
        assert_eq!(q.try_pop(), Some(2));
        assert_eq!(q.try_pop(), Some(3));
        assert_eq!(q.try_pop(), None);
    }

    #[test]
    fn blocking_pop_wakes_on_push() {
        let q = TaskQueue::<i32>::new();
        let q2 = Arc::clone(&q);
        let handle = thread::spawn(move || q2.pop());

        thread::sleep(Duration::from_millis(50));
        q.push(42);

        assert_eq!(handle.join().unwrap(), Some(42));
    }

    #[test]
    fn shutdown_unblocks_waiters() {
        let q = TaskQueue::<i32>::new();
        let q2 = Arc::clone(&q);
        let handle = thread::spawn(move || q2.pop());

        thread::sleep(Duration::from_millis(50));
        q.shutdown();

        assert_eq!(handle.join().unwrap(), None);
    }
}