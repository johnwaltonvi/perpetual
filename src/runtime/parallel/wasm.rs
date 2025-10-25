use rayon::ThreadPoolBuildError;

pub struct ThreadPool;

impl ThreadPool {
    pub fn scope<'scope, F, R>(&self, func: F) -> R
    where
        F: FnOnce(&rayon::Scope<'scope>) -> R + Send,
        R: Send,
    {
        rayon::scope(func)
    }

    pub fn current_num_threads(&self) -> usize {
        rayon::current_num_threads()
    }
}

pub struct ThreadPoolBuilder {
    num_threads: Option<usize>,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        Self { num_threads: None }
    }

    pub fn num_threads(mut self, threads: usize) -> Self {
        self.num_threads = Some(threads);
        self
    }

    pub fn build(self) -> Result<ThreadPool, ThreadPoolBuildError> {
        let _ = self.num_threads;
        Ok(ThreadPool)
    }
}
