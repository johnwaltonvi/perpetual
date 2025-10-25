use rayon::ThreadPoolBuildError;

pub struct ThreadPool {
    inner: rayon::ThreadPool,
}

impl ThreadPool {
    pub fn scope<'scope, F, R>(&self, func: F) -> R
    where
        F: FnOnce(&rayon::Scope<'scope>) -> R + Send,
        R: Send,
    {
        self.inner.scope(func)
    }

    pub fn current_num_threads(&self) -> usize {
        self.inner.current_num_threads()
    }
}

impl From<rayon::ThreadPool> for ThreadPool {
    fn from(inner: rayon::ThreadPool) -> Self {
        Self { inner }
    }
}

pub struct ThreadPoolBuilder {
    inner: rayon::ThreadPoolBuilder,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        Self {
            inner: rayon::ThreadPoolBuilder::new(),
        }
    }

    pub fn num_threads(mut self, threads: usize) -> Self {
        self.inner = self.inner.num_threads(threads);
        self
    }

    pub fn build(self) -> Result<ThreadPool, ThreadPoolBuildError> {
        self.inner.build().map(ThreadPool::from)
    }
}
