pub struct ThreadPool;

#[derive(Debug)]
pub struct PoolCreationError {
    pub details: String,
}

impl PoolCreationError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self
    }

    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::new(
                "ThreadPool size must be greater than 0",
            ))
        }
        Ok(Self::new(size))
    }

    #[allow(unused_variables)]
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
    }
}