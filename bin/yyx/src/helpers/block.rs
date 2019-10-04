use futures01::prelude::*;
use futures_cpupool::CpuPool;
use lazy_static::lazy_static;

lazy_static! {
  static ref POOL: CpuPool = { CpuPool::new_num_cpus() };
}

pub fn block<F, R>(f: F) -> impl Future<Item = R::Item, Error = R::Error>
where
  F: FnOnce() -> R + Send + 'static,
  R: IntoFuture + 'static,
  R::Future: Send + 'static,
  R::Item: Send + 'static,
  R::Error: Send + 'static,
{
  POOL.spawn_fn(f)
}
