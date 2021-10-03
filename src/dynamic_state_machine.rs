use std::collections::HashMap;
use std::hash::Hash;

use crate::error::{Result, StateMachineError};
use crate::state::State;
use crate::event::Event;

pub type TransitionCallback<S, E> = dyn Fn(&State<S>, &Event<E>) -> Option<State<S>>;
pub type BoxTransitionCallback<S, E> = Box<TransitionCallback<S, E>>;

pub struct DynamicTransition<S, E> {
    curr_state: State<S>,
    event: Event<E>,
    callback: BoxTransitionCallback<S, E>,
}

impl<S, E> DynamicTransition<S, E> {
    pub fn new<C>(
        curr_state: State<S>,
        event: Event<E>,
        callback: C,
    ) -> Self
        where C: Fn(&State<S>, &Event<E>) -> Option<State<S>> + 'static
    {
        DynamicTransition {
            curr_state,
            event,
            callback: Box::new(callback),
        }
    }

    pub fn next(&self) -> Option<State<S>> {
        (self.callback)(&self.curr_state, &self.event)
    }
}

#[derive(Hash, PartialEq, Eq)]
struct StateEvent {
    pub(crate) state_id: usize,
    pub(crate) event_id: usize,
}

pub struct DynamicStateMachine<C, E> {
    curr: Option<State<C>>,
    body: HashMap<StateEvent, DynamicTransition<C, E>>,
}

impl<C, E> DynamicStateMachine<C, E> {
    pub fn new(start_state: State<C>) -> Self {
        DynamicStateMachine {
            curr: Some(start_state),
            body: HashMap::new(),
        }
    }

    pub fn insert_transition(mut self, transition: DynamicTransition<C, E>) -> Self {
        self.body.insert(
            StateEvent {
                state_id: transition.curr_state.id,
                event_id: transition.event.id,
            },
            transition,
        );

        self
    }

    pub fn event(&mut self, input: &Event<E>) -> Result<Option<&State<C>>> {
        let key = StateEvent {
            state_id: self
                .curr
                .as_ref()
                .ok_or(StateMachineError::UnknownCurrState)?
                .id,
            event_id: input.id,
        };

        self.curr = self
            .body
            .get(&key)
            .ok_or(StateMachineError::UnknownNextState)?
            .next();

        Ok(self.curr.as_ref())
    }

    pub fn current_state(&self) -> Option<&State<C>> {
        self.curr.as_ref()
    }
}
