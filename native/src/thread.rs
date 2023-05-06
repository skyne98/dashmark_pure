use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use std::cell::RefCell;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use futures::stream::FuturesUnordered;
#[cfg(not(target_arch = "wasm32"))]
pub use std::thread;
#[cfg(target_arch = "wasm32")]
pub use wasm_thread as thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
    done_receiver: Receiver<()>,
    job_counter: RefCell<u32>,
    to_initialize: Arc<AtomicU32>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let (done_sender, done_receiver) = mpsc::channel();
        let done_sender = done_sender;
        let to_initialize = Arc::new(AtomicU32::new(size as u32));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                done_sender.clone(),
                to_initialize.clone(),
            ));
        }

        ThreadPool {
            workers,
            sender,
            done_receiver,
            job_counter: RefCell::new(0),
            to_initialize: to_initialize,
        }
    }

    pub fn initialized(&self) -> bool {
        self.to_initialize.load(Ordering::Relaxed) == 0
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        *self.job_counter.borrow_mut() += 1;
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn par_iter<F, T>(&self, arr: &[T], func: F) -> Result<()>
    where
        F: Fn(&T) + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        let arr_len = arr.len();
        let chunks_count = self.workers.len();
        let chunk_size = arr_len / chunks_count;
        let remaining_len = arr_len % chunks_count;

        let func = Arc::new(func);

        for chunk_index in 0..chunks_count {
            let slice_len = if chunk_index == chunks_count - 1 {
                chunk_size + remaining_len
            } else {
                chunk_size
            };
            let slice_start = chunk_index * chunk_size;
            let slice = &arr[slice_start..slice_start + slice_len];
            let slice_ptr = slice.as_ptr() as usize;

            let f = func.clone();
            self.execute(move || {
                let slice = unsafe { std::slice::from_raw_parts(slice_ptr as *const T, slice_len) };
                for item in slice {
                    f(item);
                }
            });
        }

        self.wait_for_completion();
        Ok(())
    }

    pub fn par_iter_mut<F, T>(&self, arr: &mut [T], func: F) -> Result<()>
    where
        F: Fn(&mut T) + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        let arr_len = arr.len();
        let chunks_count = self.workers.len();
        let chunk_size = arr_len / chunks_count;
        let remaining_len = arr_len % chunks_count;

        let func = Arc::new(func);

        for chunk_index in 0..chunks_count {
            let slice_len = if chunk_index == chunks_count - 1 {
                chunk_size + remaining_len
            } else {
                chunk_size
            };
            let slice_start = chunk_index * chunk_size;
            let slice = &arr[slice_start..slice_start + slice_len];
            let slice_ptr = slice.as_ptr() as usize;

            let f = func.clone();
            self.execute(move || {
                let slice =
                    unsafe { std::slice::from_raw_parts_mut(slice_ptr as *mut T, slice_len) };
                for item in slice {
                    f(item);
                }
            });
        }

        self.wait_for_completion();
        Ok(())
    }

    pub fn wait_for_completion(&self) {
        loop {
            let mut job_counter = self.job_counter.borrow_mut();
            if *job_counter == 0 {
                break;
            }

            if let Ok(_) = self.done_receiver.try_recv() {
                *job_counter = *job_counter - 1;
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                #[cfg(not(target_arch = "wasm32"))]
                thread.join().unwrap();

                #[cfg(target_arch = "wasm32")]
                pollster::block_on(thread.join_async()).unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Message>>>,
        done_sender: Sender<()>,
        to_initialize: Arc<AtomicU32>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            log::debug!("Worker {} started", id);
            to_initialize.fetch_sub(1, Ordering::Relaxed);
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job();
                        done_sender.send(()).unwrap();
                    }
                    Message::Terminate => {
                        log::debug!("Worker {} was told to terminate", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

#[cfg(test)]
mod tests_thread {
    use super::ThreadPool;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_zero_thread_pool_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut count = counter.lock().unwrap();
                *count += 1;
                std::thread::sleep(Duration::from_millis(10));
            });
        }

        // Sleep to ensure tasks are completed before checking the counter
        std::thread::sleep(Duration::from_secs(1));

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_wait_for_completion() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                println!("Executing job");
                let mut count = counter.lock().unwrap();
                *count += 1;
                std::thread::sleep(std::time::Duration::from_millis(10));
            });
        }

        let start_time = std::time::Instant::now();
        pool.wait_for_completion();
        let duration = start_time.elapsed();

        assert_eq!(*counter.lock().unwrap(), 10);
        assert!(duration >= std::time::Duration::from_millis(50));
    }

    #[test]
    fn test_for_each_mut() {
        let pool = ThreadPool::new(4);

        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let squared_numbers_result = pool.par_iter_mut(&mut numbers[..], |n| *n *= 2);

        assert!(squared_numbers_result.is_ok());
        assert_eq!(numbers, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    }
}
