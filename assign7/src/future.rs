#![feature(core_intrinsics)]
use take_mut;
use std::intrinsics::{uninit, unreachable};
use std::mem;
use std::borrow::Borrow;
use core::intrinsics;
use std::ops::Deref;
use proc_macro::Spacing::Joint;
use std::mem::take;
use future::Join::Done;

/*
 * Core futures interface.
 */

#[derive(Debug)]
pub enum Poll<T> {
  Ready(T),
  NotReady,
}

pub trait Future: Send {
  type Item: Send;
  fn poll(&mut self) -> Poll<Self::Item>;
}

/*
 * Example implementation of a future for an item that returns immediately.
 */

// Container for the state of the future.
pub struct Immediate<T> {
  t: Option<T>,
}

// Constructor to build the future. Note that the return type just says
// "this produces a future", not specifying concretely the type Immediate.
pub fn immediate<T>(t: T) -> impl Future<Item = T>
where
  T: Send,
{
  Immediate { t: Some(t) }
}

// To treat Immediate as a future, we have to implement poll. Here it's
// relatively simple, since we return immediately with a Poll::Ready.
impl<T> Future for Immediate<T>
where
  T: Send,
{
  type Item = T;

  fn poll(&mut self) -> Poll<Self::Item> {
    Poll::Ready(self.t.take().unwrap())
  }
}

/*
 * Example implementation of a future combinator that applies a function to
 * the output of a future.
 */

struct Map<Fut, Fun> {
  fut: Fut,
  fun: Option<Fun>,
}

pub fn map<T, Fut, Fun>(fut: Fut, fun: Fun) -> impl Future<Item = T>
where
  T: Send,
  Fut: Future,
  Fun: FnOnce(Fut::Item) -> T + Send,
{
  Map {
    fut,
    fun: Some(fun),
  }
}

impl<T, Fut, Fun> Future for Map<Fut, Fun>
where
  T: Send,
  Fut: Future,
  Fun: FnOnce(Fut::Item) -> T + Send,
{
  type Item = T;

  fn poll(&mut self) -> Poll<Self::Item> {
    match self.fut.poll() {
      Poll::NotReady => Poll::NotReady,
      Poll::Ready(s) => {
        let f = self.fun.take();
        Poll::Ready(f.unwrap()(s))
      }
    }
  }
}


/*
 * Part 1a - Join
 */

// A join of two futures is a state machine depending on which future is
// completed, represented as an enum.
pub enum Join<F, G>
where
  F: Future,
  G: Future,
{
  BothRunning(F, G),
  FirstDone(F::Item, G),
  SecondDone(F, G::Item),
  Done,
}

// When a join is created, we start by assuming neither child future
// has completed.
pub fn join<F, G>(f: F, g: G) -> impl Future<Item = (F::Item, G::Item)>
where
  F: Future,
  G: Future,
{
  Join::BothRunning(f, g)
}

impl<F, G> Future for Join<F, G>
where
  F: Future,
  G: Future,
{
  type Item = (F::Item, G::Item);


  fn poll(&mut self) -> Poll<Self::Item> {
    match self {
      Join::BothRunning(future1, future2_mut) => {
        match (future1.poll(), future2_mut.poll()) {
          (Poll::NotReady, Poll::NotReady)  => Poll::NotReady,
          (Poll::Ready(completed1), Poll::NotReady) => {
            take_mut::take(self, |join| {
              match join {
                Join::BothRunning(future1, future2) => {
                  Join::FirstDone(completed1, future2)
                },
                _ => unreachable!()
              }
            });
            Poll::NotReady
          }
          (Poll::NotReady, Poll::Ready(completed2)) => {
            take_mut::take(self, |join| {
              match join {
                Join::BothRunning(future1, future2) => {
                  Join::SecondDone(future1, completed2)
                },
                _ => unreachable!()
              }
            });
            Poll::NotReady
          },
          (Poll::Ready(completed1), Poll::Ready(completed2)) => {
            *self = Join::Done;
            Poll::Ready ((completed1, completed2))
          }
        }
      },
      Join::SecondDone(future, completed_mut) => {
        let mut result= Poll::NotReady;
        match future.poll() {
          Poll::NotReady => Poll::NotReady,
          Poll::Ready(first_completed) => {
            let old_join = mem::replace(self, Join::Done);
            match old_join {
              Join::SecondDone(_, second_completed) => Poll::Ready( (first_completed, second_completed)),
              _ => Poll::NotReady
            }
          }
        };
        result
      },
      Join::FirstDone(first_completed_mut, future) => {
        let mut result : Poll<Self::Item> = Poll::NotReady;
        match future.poll() {
          Poll::NotReady => Poll::NotReady,
          Poll::Ready(second_completed) => {
            let old_join= mem::replace(self, Join::Done);
            match old_join {
              Join::FirstDone(first_completed, _) => Poll::Ready( (first_completed, second_completed)),
              _ => Poll::NotReady
            }
          }
        };
        result
      },
      Join::Done => unreachable!("Should not reach here when done")
    }
  }
}

/*
 * Part 1b - AndThen
 */

// The AndThen state machine depends on which future is currently running.
pub enum AndThen<Fut1, Fut2, Fun> {
  First(Fut1, Fun),
  Second(Fut2),
  Done,
}

pub fn and_then<Fut1, Fut2, Fun>(fut: Fut1, fun: Fun)
                                 -> impl Future<Item = Fut2::Item>
where
  Fut1: Future,
  Fut2: Future,
  Fun: FnOnce(Fut1::Item) -> Fut2 + Send,
{
  AndThen::First(fut, fun)
}

impl<Fut1, Fut2, Fun> Future for AndThen<Fut1, Fut2, Fun>
where
  Fut1: Future,
  Fut2: Future,
  Fun: FnOnce(Fut1::Item) -> Fut2 + Send,
{
  type Item = Fut2::Item;

  fn poll(&mut self) -> Poll<Self::Item> {
    match self {
      AndThen::Second(completed_future) =>
        completed_future.poll(),
      AndThen::First(future, f_mut) => {
        match future.poll() {
            Poll::NotReady => Poll::NotReady,
            Poll::Ready(ready_value) => {
              take_mut::take(self, |value| {
                match value {
                  AndThen::First(_, f) =>
                  AndThen::Second(f (ready_value)),
                  _ => unreachable!()
                }
              });
              Poll::NotReady

          }
        }
      },
      AndThen::Done =>
        unimplemented!()

    }
  }
}
