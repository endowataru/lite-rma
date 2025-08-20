pub trait Sched: Send + Sync {
    fn yield_now(&self) -> impl std::future::Future<Output = ()> + Send;

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send,
        T: Send;
}

pub struct OsSched {}

impl Sched for OsSched {
    fn yield_now(&self) -> impl std::future::Future<Output = ()> + Send {
        async move {
            std::thread::yield_now();
        }
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send,
        T: Send,
    {
        async_std::task::block_on(future)
    }
}
