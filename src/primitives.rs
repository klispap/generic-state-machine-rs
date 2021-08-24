use crate::error::{ContexError, Error, Result};
use derivative::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Mutex;

pub type TransitionFunction<S, E> = fn(&StateMachine<S, E>, E) -> S;

/// The primitive state machine type that holds all context
///  * List of available states
///  * Maps of input events per state. An event not in the list is ignored by default
///  * Maps of transition functions per input event
/// It also holds the live status of the state machine:
///  * Queue with incoming input events to be processed
///  * Current state
///
/// **Important Note:**
/// This type is used to implement the async (and later the sync) versions of the generic state machine
/// You should not use this type for most of use cases.
/// Use the async (or sync) implementations instead!
#[derive(Debug)]
pub struct StateMachine<S, E>
where
    S: Clone + std::hash::Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    states: Vec<S>,
    current: Mutex<Option<S>>,
    pub transitions: HashMap<S, StateMachineTransitions<S, E>>,
}

/// Helper structure to bypass Debug trait limitations on function items
#[derive(Derivative)]
#[derivative(Debug)]
pub struct StateMachineTransitions<S, E>
where
    S: Clone + Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    #[derivative(Debug(format_with = "my_fmt_fn"))]
    map: HashMap<E, TransitionFunction<S, E>>,
}

/// Helper function to bypass Debug trait limitations on function items
fn my_fmt_fn<K: std::fmt::Debug, V>(
    t: &HashMap<K, V>,
    f: &mut std::fmt::Formatter,
) -> std::result::Result<(), std::fmt::Error> {
    f.debug_list().entries(t.keys()).finish()
}

impl<S, E> Default for StateMachine<S, E>
where
    S: Clone + Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, E> StateMachine<S, E>
where
    S: Clone + Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    /// Create a new empty state machine
    ///  The state machine needs to be confifures properly using:
    ///   * [add_state()](StateMachine::add_states)
    ///   * [add_transition()](StateMachine::add_transition)
    ///   * [set_state()](StateMachine::set_state)
    ///  The above methods can be chained in a single command, like so:
    ///  ```rust
    ///     use generic_state_machine::primitives::StateMachine;
    ///
    ///     let mut fsm = StateMachine::<String, String>::new();
    ///     fsm.add_states(&mut vec![
    ///         "alpha".to_string(),
    ///         "beta".to_string(),
    ///         "gamma".to_string(),
    ///         ])
    ///         .add_transition("alpha".to_string(), "beta".to_string(), |_, _| {
    ///             "beta".to_string()
    ///         })
    ///         .initial_state("alpha".to_string());
    ///
    ///     println!("{:?}", fsm);
    pub fn new() -> Self {
        StateMachine {
            states: vec![],
            current: Mutex::new(None),
            transitions: std::collections::HashMap::new(),
        }
    }

    /// Get the current state
    pub fn current_state(&self) -> Result<S> {
        match &self.current.lock().unwrap().clone() {
            Some(s) => Ok(s.clone()),
            None => Err(Error::NoCurrentState),
        }
    }

    pub fn initial_state(&mut self, state: S) -> Result<&mut Self> {
        if self.current.lock()?.is_some() {
            Err(Error::InitialStateDoubleSet)
        } else {
            *self.current.lock().unwrap() = Some(state);
            Ok(self)
        }
    }

    /// Force the current state to the state provided. Used to set the initial state
    pub(crate) fn set_state(&mut self, state: S) -> &mut Self {
        // let mut test = Some(1);

        // test = match test {
        //     Some(_) => unreachable!(),
        //     None => Some(2),
        // };

        *self.current.lock().unwrap() = Some(state);
        self
    }

    /// Add the provided states to the list of available states
    pub fn add_states(&mut self, states: &[S]) -> &mut Self {
        self.states.extend_from_slice(states);
        self
    }

    /// Add the provided function as the transition function to be executed if the machine
    /// is in the provided state and receives the provided input event
    pub fn add_transition(
        &mut self,
        state: S,
        event: E,
        function: TransitionFunction<S, E>,
    ) -> &mut Self {
        if let Some(transitions) = self.transitions.get_mut(&state) {
            transitions.map.insert(event, function);
        } else {
            self.transitions.insert(state, {
                let mut transitions = StateMachineTransitions {
                    map: std::collections::HashMap::new(),
                };
                transitions.map.insert(event, function);
                transitions
            });
        }
        self
    }

    /// Executes the received event
    /// Checks if a transition function is provided for the current state and for the incoming event
    /// It calls the function and sets the state to the output of the transition function.
    /// If no transition function is available, the incoming event is dropped.
    pub fn execute(&mut self, event: E) -> Result<S> {
        if let Some(events) = self.transitions.get(&self.current_state()?) {
            if let Some(&function) = events.map.get(&event) {
                let res = function(self, event);
                self.set_state(res);
            } else {
                return Err(ContexError::EventNotMachingState(self.current_state()?, event).into());
            }
        } else {
            unreachable!();
        }

        let current_state = self.current_state()?;
        Ok(current_state)
    }
}
