use std::collections::HashMap;
use std::hash::Hash;

use crate::error::{Result, StateMachineError};
use crate::state::State;
use crate::event::Event;

pub type TransitionCallback<S, E> = dyn Fn(&S, &E) -> Option<S>;

pub struct Transition<S, E> {
    curr_state: State<S>,
    event: Event<E>,
    callback: Box<TransitionCallback<State<S>, Event<E>>>,
}

impl<S, E> Transition<S, E> {
    pub fn new<C>(
        curr_state: State<S>,
        event: Event<E>,
        callback: C,//TransitionCallback<State<S>, Event<E>>,
    ) -> Self 
        where C: Fn(&State<S>, &Event<E>) -> Option<State<S>> + 'static
    {
        Transition {
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

pub struct StateMachine<C, E> {
    curr: Option<State<C>>,
    body: HashMap<StateEvent, Transition<C, E>>,
}

impl<C, E> StateMachine<C, E> {
    pub fn new(start_state: State<C>) -> Self {
        StateMachine {
            curr: Some(start_state),
            body: HashMap::new(),
        }
    }

    pub fn insert_transition(mut self, transition: Transition<C, E>) -> Self {
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
            state_id: self.curr.as_ref().ok_or(StateMachineError::UnknownCurrState)?.id,
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
