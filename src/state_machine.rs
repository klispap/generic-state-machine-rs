use crate::primitives::StateMachine;
use anyhow::{anyhow, bail, Result};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::{Send, Sync};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task;

/// The asychronous version of the generic state machine
///
#[derive(Debug)]
pub struct AsyncStateMachine<S, E>
where
    S: Default + Clone + std::hash::Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    pub state_machine: Arc<Mutex<StateMachine<S, E>>>,
    queue_size: usize,
    push_events: Option<Sender<E>>,
}

impl<S: 'static, E: 'static> AsyncStateMachine<S, E>
where
    S: Default + Clone + Hash + Eq + Debug + Send + Sync,
    E: Hash + Eq + Debug + Send + Sync,
{
    /// Create a new empty and *unconfigured* state machine
    pub fn new(queue_size: usize) -> Self {
        AsyncStateMachine {
            state_machine: Arc::new(Mutex::new(StateMachine::new())),
            queue_size,
            push_events: None,
        }
    }

    /// Start the event handler task
    pub async fn start(&mut self) -> Result<()> {
        if self.push_events.is_some() {
            bail!("State Machine is already running!")
        }

        let (sender, receiver) = channel::<E>(self.queue_size);
        self.push_events = Some(sender);

        // Spawn the loop task that keeps polling message from the queue
        task::spawn(state_machine_task(self.state_machine.clone(), receiver));

        Ok(())
    }

    pub async fn push_event(&mut self, event: E) -> Result<()> {
        match &self.push_events {
            None => bail!("State machine is not started!"),
            Some(sender) => sender
                .send(event)
                .await
                .map_err(|_| anyhow!("Failed to push event in state machine!")),
        }
    }

    /// Get the current state
    pub async fn current_state(&self) -> S {
        self.state_machine.lock().await.current_state()
    }

    /// Force the current state to the state provided. Used to set the initial state
    pub async fn set_state(&mut self, state: S) -> &mut Self {
        self.state_machine.lock().await.set_state(state);
        self
    }

    /// Add the provided states to the list of available states
    pub async fn add_states(&mut self, states: &mut Vec<S>) -> &mut Self {
        self.state_machine.lock().await.add_states(states);
        self
    }

    /// Add the provided function as the transition function to be executed if the machine
    /// is in the provided state and receives the provided input event
    pub async fn add_transition(
        &mut self,
        current_state: S,
        event: E,
        function: crate::primitives::TransitionFunction<S, E>,
    ) -> &mut Self {
        self.state_machine
            .lock()
            .await
            .add_transition(current_state, event, function);
        self
    }
}

async fn state_machine_task<S, E>(
    state_machine: Arc<Mutex<StateMachine<S, E>>>,
    mut events: Receiver<E>,
) where
    S: Default + Clone + std::hash::Hash + Eq + Debug,
    E: Hash + Eq + Debug,
{
    loop {
        // Poll next event in queue
        let next_event = events.recv().await;

        if next_event.is_none() {
            break;
        }

        state_machine.lock().await.execute(next_event.unwrap());
    }
}
