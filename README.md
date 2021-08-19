## Summary
A simple library that allows to create Moore or Mealy state machines
It is designed around the [`StateMachine<S,E>`](state_machine::StateMachine) type and allows the use of
custom transition functions.

## Requirements:
 * All states need to be of the same type `S`
 * All events (inputs) need to be of the same type `E`
 * Transition functions are executed over `self` (to have access to internal lists of states or events)
   and need to follow the prototype:
   ```rust, ignore
    fn(&mut StateMachine<S, E>, E) -> S;
   ```
   where input E is the trigger event and the return value is the new state.
 * Output generation is left to the user to allow the implementation to be as generic as possible.
   You can print or call exernal functions to produce the desired output
 
## Implement Moore state machine:
 * Define transition functions that calculate the next state and use that to produce any outputs

## Implement Mealy state machine:
 * Define transition functions that calculate the next state and use that together with the input event E to produce any outputs

### A Basic Example of a Asynchronous State Machine:

```ignore
           +------------+
  Event: 1 |            | +----+
           |            V |    | Event: 1
     +->[false]       [true]<--+
     |   | ^            |
     +---+ |            | Event: 0
 Event: 0  +------------+
```

```rust
use state_machine::state_machine::AsyncStateMachine;

async fn main() -> Result<()> {
let mut fsm = AsyncStateMachine::<bool, usize>::new(5);
    fsm.add_states(&mut vec![true, false])
      .await
      .add_transition(true, 0, |_fsm, _event| false)
      .await
      .add_transition(true, 1, |_fsm, _event| true)
      .await
      .add_transition(false, 0, |_fsm, _event| false)
      .await
      .add_transition(false, 1, |_fsm, _event| true)
      .await
      .set_state(false)
      .await;

    fsm.start().await.unwrap();

    println!("State Machine under test: {:?}", fsm);

    fsm.push_event(0).await.unwrap();
    assert_eq!(fsm.current_state().await, false);

    fsm.push_event(1).await.unwrap();
    assert_eq!(fsm.current_state().await, true);

    fsm.push_event(1).await.unwrap();
    assert_eq!(fsm.current_state().await, true);

    fsm.push_event(0).await.unwrap();
    assert_eq!(fsm.current_state().await, false);

    fsm.push_event(0).await.unwrap();
    assert_eq!(fsm.current_state().await, false);
}
```

### A Basic Example of a State Machine (using the primitive type):

```ignore
           +---->[1]----+
  Event: 1 |            | Event: 2
           |            V
          [3]          [2]
           ^            |
           |            | Event: 3
           +------------+
```
```rust
    use state_machine::primitives::StateMachine;

    // Define a transition function. It can as general as we want!
    fn tf(_fsm: &mut StateMachine<i32, i32>, event: i32) -> i32 {
    match event {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => panic!(), // Invalid transition
        }
    }

    let mut fsm = StateMachine::<i32, i32>::new();
    fsm.add_states(&mut vec![1, 2, 3]);

    // We can even use captures as transition functions!
    fsm.add_transition(1, 2, |_fsm, _event| {
        // We are in state 1! Go to state 2! Ignore Input!
        2
    });

    // If you prefer you can chain configuration methods together
    fsm.add_transition(2, 3, tf)
      .add_transition(3, 1, tf)
      .set_state(1);

    println!("{:?}", fsm);

    assert_eq!(&1, fsm.execute(1));
    assert_eq!(&2, fsm.execute(2));
    assert_eq!(&3, fsm.execute(3));
```
