use derivative::*;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

type TransitionFunction<S, E> = fn(&mut StateMachine<S, E>, E) -> S;

/// The state machine type that holds all context
///  * List of available states
///  * Maps of input events per state. An event not in the list is ignored by default
///  * Maps of transition functions per input event
/// It also holds the live status of the state machine:
///  * Queue with incoming input events to be processed
///  * Current state
#[derive(Debug)]
pub struct StateMachine<S, E>
where
    S: Default + Clone + std::hash::Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    states: Vec<S>,
    current: S,
    events: VecDeque<E>,
    transitions: HashMap<S, StateMachineTransitions<S, E>>,
}

/// Helper structure to bypass Debug trait limitations on function items
#[derive(Derivative)]
#[derivative(Debug)]
struct StateMachineTransitions<S, E>
where
    S: Default + Clone + Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    #[derivative(Debug(format_with = "my_fmt_fn"))]
    map: HashMap<E, TransitionFunction<S, E>>,
}

/// Helper function to bypass Debug trait limitations on function items
fn my_fmt_fn<K: std::fmt::Debug, V>(
    t: &HashMap<K, V>,
    f: &mut std::fmt::Formatter,
) -> Result<(), std::fmt::Error> {
    f.debug_list().entries(t.keys()).finish()
}

impl<S, E> Default for StateMachine<S, E>
where
    S: Default + Clone + Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, E> StateMachine<S, E>
where
    S: Default + Clone + Hash + Eq + Debug,
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
    ///         .set_state("alpha".to_string());
    ///
    ///     println!("{:?}", fsm);
    pub fn new() -> Self {
        StateMachine {
            states: vec![],
            current: S::default(),
            events: std::collections::VecDeque::new(),
            transitions: std::collections::HashMap::new(),
        }
    }

    /// Get the current state
    pub fn current_state(&self) -> &S {
        &self.current
    }

    /// Force the current state to the state provided. Used to set the initial state
    pub fn set_state(&mut self, state: S) -> &mut Self {
        self.current = state;
        self
    }

    /// Add the provided states to the list of available states
    pub fn add_states(&mut self, states: &mut Vec<S>) -> &mut Self {
        self.states.append(states);
        self
    }

    /// Add the provided function as the transition function to be executed if the machine
    /// is in the provided state and receives the provided input event
    pub fn add_transition(
        &mut self,
        current_state: S,
        event: E,
        function: fn(&mut Self, E) -> S,
    ) -> &mut Self {
        if let Some(events) = self.transitions.get_mut(&current_state) {
            events.map.insert(event, function);
        } else {
            self.transitions.insert(current_state, {
                let mut map = StateMachineTransitions {
                    map: std::collections::HashMap::new(),
                };
                map.map.insert(event, function);
                map
            });
        }
        self
    }

    /// Executes the received event
    /// Checks if a transition function is provided for the current state and for the incoming event
    /// It calls the function and sets the state to the output of the transition function.
    /// If no transition function is available, the incoming event is dropped.
    pub fn execute(&mut self, event: E) -> &S {
        if let Some(events) = self.transitions.get(self.current_state()) {
            if let Some(&function) = events.map.get(&event) {
                let res = function(self, event);
                self.set_state(res);
            }
        }
        self.current_state()
    }
}
