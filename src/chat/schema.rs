use std::sync::{atomic::AtomicUsize, Arc};

#[derive(Clone)]
pub struct UserCnt(Arc<AtomicUsize>);
impl UserCnt {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }
    fn add_user(&self) -> usize {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
    }
    fn remove_user(&self) -> usize {
        self.0.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) - 1
    }
}
