use std::mem;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use future::{Future, Poll};
use std::thread::JoinHandle;

/*
 * Core executor interface.
 */

pub trait Executor {
  fn spawn<F>(&mut self, f: F)
  where
    F: Future<Item = ()> + 'static;
  fn wait(&mut self);
}


/*
 * Example implementation of a naive executor that executes futures
 * in sequence.
 */

pub struct BlockingExecutor;

impl BlockingExecutor {
  pub fn new() -> BlockingExecutor {
    BlockingExecutor
  }
}

impl Executor for BlockingExecutor {
  fn spawn<F>(&mut self, mut f: F)
  where
    F: Future<Item = ()>,
  {
    loop {
      if let Poll::Ready(()) = f.poll() {
        break;
      }
    }
  }

  fn wait(&mut self) {}
}

/*
 * Part 2a - Single threaded executor
 */

pub struct SingleThreadExecutor {
  futures: Vec<Box<dyn Future<Item = ()>>>,
}

impl SingleThreadExecutor {
  pub fn new() -> SingleThreadExecutor {
    SingleThreadExecutor { futures: vec![] }
  }
}

impl Executor for SingleThreadExecutor {
  fn spawn<F>(&mut self, mut f: F)
  where
    F: Future<Item = ()> + 'static,
  {
    match f.poll() {
      Poll::Ready(_) => (),
      Poll::NotReady => self.futures.push(
        Box::new(f)
      )
    }
  }

  fn wait(&mut self) {
    while not (self.futures.is_empty()) {
      let mut result_vector = vec![];
      for (i, future) in self.futures.iter_mut().enumerate() {
        match future.poll() {
          Poll::Ready(_) => (),
          Poll::NotReady => result_vector.push(i)
        }
      }
      result_vector.reverse();
      for i in result_vector {
        self.futures.remove(i);
        ()
      }
    }


  }
}

pub struct MultiThreadExecutor {
  sender: mpsc::Sender<Option<Box<dyn Future<Item = ()>>>>,
  threads: Vec<thread::JoinHandle<()>>,
}

impl MultiThreadExecutor {
  pub fn new(num_threads: i32) -> MultiThreadExecutor {
    let threads = vec![];
    let (sender, receiver )= mpsc::channel();

    let iter = (0..num_threads);

    for i in (0..num_threads) {
      let thread = thread::spawn(|| {
          let mut single_thread = SingleThreadExecutor::new();
          single_thread.wait()
      });

      threads.push(thread.)
    }
  }
}

impl Executor for MultiThreadExecutor {
  fn spawn<F>(&mut self, f: F)
  where
    F: Future<Item = ()> + 'static,
  {
    self.sender.send( f).unwrap();
  }

  fn wait(&mut self) {
    for thread in self.threads {
      thread.join().unwrap();
    }

  }
}
