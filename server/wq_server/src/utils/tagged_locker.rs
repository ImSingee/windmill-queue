use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};

#[derive(Clone)]
pub struct TaggedLocker<K: std::hash::Hash + std::cmp::Eq> {
    locks: DashMap<K, Arc<Mutex<()>>>,
}

impl<K> TaggedLocker<K>
    where
        K: Eq + std::hash::Hash + std::cmp::Eq,
{
    pub fn new() -> Self {
        Self {
            locks: DashMap::new(),
        }
    }

    pub async fn lock_for(&self, tag: K) -> OwnedMutexGuard<()> {
        let lock = self
            .locks
            .entry(tag)
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .value()
            .clone();

        lock.lock_owned().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    enum T {
        Test1,
        Test2,
    }

    /// try to acquire the lock in 100ms, return true if it success
    async fn try_lock(tag_lock: Arc<TaggedLocker<T>>, tag: T) -> bool {
        let handle = tokio::spawn(async move { tag_lock.lock_for(tag).await });

        let result = timeout(Duration::from_millis(100), handle).await;

        result.is_ok()
    }

    #[tokio::test]
    async fn test_same_tag_cannot_lock_concurrently() {
        let tag_lock = Arc::new(TaggedLocker::new());
        let tag = T::Test1;

        // acquire lock
        let lock = tag_lock.lock_for(tag).await;

        // acquire lock again (should wait)
        let success = try_lock(Arc::clone(&tag_lock), tag).await;
        assert!(!success, "The lock was acquired before it should have been");

        // drop lock1, to allow lock_future to acquire the lock
        drop(lock);

        // acquire lock again (should success now)
        let success = try_lock(Arc::clone(&tag_lock), tag).await;
        assert!(success, "The lock was not acquired after it was released");
    }

    #[tokio::test]
    async fn test_different_tags_can_lock_concurrently() {
        let tag_lock = Arc::new(TaggedLocker::new());

        // acquire lock1
        let lock = tag_lock.lock_for(T::Test1).await;

        // try to acquire lock2 (should success)
        let success = try_lock(Arc::clone(&tag_lock), T::Test2).await;
        assert!(
            success,
            "The lock for a different tag was not acquired concurrently"
        );

        // make sure lock is live
        _ = lock;
    }
}
