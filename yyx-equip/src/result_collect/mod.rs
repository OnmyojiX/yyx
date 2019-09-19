use crossbeam_channel::{Receiver, Sender};
use std::thread::{self, JoinHandle};

mod container;

use self::container::Container;

const CHANNEL_CAP: usize = 100;

#[derive(Debug)]
pub struct ResultCollect<T> {
  sender: Sender<Req<T>>,
  res_receiver: Receiver<Res<T>>,
  worker: Option<JoinHandle<()>>,
}

impl<T> ResultCollect<T>
where
  T: Ord + Send + Clone + 'static,
{
  pub fn new(capacity: usize) -> ResultCollect<T> {
    let (sender, receiver) = crossbeam_channel::bounded(CHANNEL_CAP);
    let (res_sender, res_receiver) = crossbeam_channel::bounded(0);
    let mut container = Container::<T>::with_capacity(capacity);
    let worker = thread::spawn(move || loop {
      match receiver.recv() {
        Ok(req) => match req {
          Req::Item(item) => {
            container.push(item);
          }
          Req::Top(n) => {
            res_sender.send(Res::Top(container.get_top(n))).unwrap();
          }
          Req::Term => {
            res_sender.send(Res::Term(container)).ok();
            break;
          }
        },
        Err(_) => break,
      }
    });
    ResultCollect {
      sender,
      res_receiver,
      worker: Some(worker),
    }
  }

  pub fn fetch_top(&self, n: usize) -> Vec<T> {
    self.sender.send(Req::Top(n)).unwrap();
    let res = self.res_receiver.recv().unwrap();
    if let Res::Top(top) = res {
      top
    } else {
      panic!("unexpected res");
    }
  }

  pub fn handle(&self) -> Handle<T> {
    Handle {
      sender: self.sender.clone(),
    }
  }

  pub fn collect(mut self) -> Vec<T> {
    self.sender.send(Req::Term).unwrap();
    let res = self.res_receiver.recv().unwrap();
    if let Res::Term(container) = res {
      if let Some(handle) = self.worker.take() {
        handle.join().unwrap()
      }
      container.into_sorted_vec()
    } else {
      panic!("unexpected res");
    }
  }
}

impl<T> std::ops::Drop for ResultCollect<T> {
  fn drop(&mut self) {
    if let Some(handle) = self.worker.take() {
      handle.join().ok();
    }
  }
}

#[derive(Debug, Clone)]
pub struct Handle<T> {
  sender: Sender<Req<T>>,
}

impl<T> Handle<T> {
  pub fn send(&self, item: T) -> Result<(), T> {
    self.sender.send(Req::Item(item)).map_err(|e| {
      if let Req::Item(item) = e.into_inner() {
        item
      } else {
        unreachable!()
      }
    })
  }
}

#[derive(Debug)]
enum Req<T> {
  Item(T),
  Top(usize),
  Term,
}

#[derive(Debug)]
enum Res<T> {
  Top(Vec<T>),
  Term(Container<T>),
}

#[cfg(test)]
mod test {
  use super::*;
  use std::thread;

  #[test]
  fn test_result_collect() {
    let c = ResultCollect::new(3);

    (0..3)
      .map(|i| {
        let handle = c.handle();
        thread::spawn(move || {
          for ii in 0..=i {
            handle.send(ii).unwrap()
          }
        })
      })
      .for_each(|handle| handle.join().unwrap());

    let top = c.fetch_top(2);
    assert_eq!(top, vec![2, 1]);

    let items: Vec<i32> = c.collect();
    assert_eq!(items, vec![2, 1, 1])
  }
}
