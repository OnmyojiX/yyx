use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Container<T> {
  inner: BinaryHeap<Reverse<T>>,
}

impl<T> Container<T>
where
  T: Ord,
{
  pub fn with_capacity(capacity: usize) -> Self {
    Container {
      inner: BinaryHeap::with_capacity(capacity),
    }
  }

  pub fn push(&mut self, item: T) {
    if self.inner.len() < self.inner.capacity() {
      self.inner.push(Reverse(item))
    } else {
      let mut min = self.inner.peek_mut().unwrap();
      let rev = Reverse(item);
      if *min > rev {
        *min = rev
      }
    }
  }

  pub fn get_top(&self, n: usize) -> Vec<T>
  where
    T: Clone,
  {
    self
      .inner
      .iter()
      .rev()
      .take(n)
      .cloned()
      .map(|i| i.0.clone())
      .collect()
  }

  pub fn into_sorted_vec(self) -> Vec<T> {
    self
      .inner
      .into_sorted_vec()
      .into_iter()
      .map(|r| r.0)
      .collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rand::seq::SliceRandom;
  use rand::thread_rng;

  #[test]
  fn test_container() {
    let mut c = Container::with_capacity(3);
    c.push(1);
    c.push(3);
    c.push(2);
    assert_eq!(c.into_sorted_vec(), vec![3, 2, 1]);

    let mut vec: Vec<u32> = (0..100).collect();
    vec.shuffle(&mut thread_rng());

    let mut c = Container::with_capacity(3);
    for i in vec {
      c.push(i)
    }
    assert_eq!(c.into_sorted_vec(), vec![99, 98, 97]);
  }
}
