use std::sync::atomic::{AtomicUsize, Ordering};

static EVENT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug)]
pub struct Event<C> {
    pub(crate) id: usize,
    content: C,
}

impl<C> Event<C> {
    pub fn new(content: C) -> Self {
        Event {
            id: EVENT_COUNTER.fetch_add(1, Ordering::SeqCst),
            content,
        }
    }

    pub fn content(&self) -> &C {
        &self.content
    }
}
