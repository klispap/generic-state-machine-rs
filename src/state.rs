// use uuid::Uuid;
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait Content<C> {
    fn content(&self) -> &C;
}

static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

// trait Id {
//     fn id(&self) -> uzi;
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State<C> {
    pub(crate) id: usize,
    content: C,
}

impl<C> State<C> {
    pub fn new(content: C) -> Self {
        State {
            id: STATE_COUNTER.fetch_add(1, Ordering::SeqCst),
            content,
        }
    }

    pub fn content(&self) -> &C {
        &self.content
    }
}
