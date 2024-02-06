use crate::utils::tagged_locker::TaggedLocker;
use std::ops::Deref;

#[derive(Clone)]
pub struct EventLocker(TaggedLocker<String>);

impl Deref for EventLocker {
    type Target = TaggedLocker<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EventLocker {
    pub fn new() -> Self {
        Self(TaggedLocker::new())
    }
}
