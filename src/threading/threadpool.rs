pub struct ThreadPool{
    size: usize,
}

impl ThreadPool{
    pub fn new(size: usize) -> Self {
        ThreadPool{
            size: size
        }
    }

    pub fn size(&self) -> &usize {
        &self.size
    }
}