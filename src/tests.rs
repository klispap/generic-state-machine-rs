#[cfg(test)]
mod tests {
    use crate::state_machine::StateMachine;
    /// Test over the following state machine:
    ///            +---->[1]----+
    ///   Event: 1 |            | Event: 2
    ///            |            V
    ///           [3]          [2]
    ///            ^            |
    ///            |            | Event: 3
    ///            +------------+
    fn tf(_fsm: &mut StateMachine<i32, i32>, event: i32) -> i32 {
        match event {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => panic!(), // Invalid transition
        }
    }
    #[test]
    fn test_i32() {
        let mut fsm = StateMachine::<i32, i32>::new();
        fsm.add_states(&mut vec![1, 2, 3])
            .add_transition(1, 2, |_fsm, _event| 2)
            .add_transition(2, 3, tf)
            .add_transition(3, 1, tf)
            .set_state(1);

        println!("{:?}", fsm);

        assert_eq!(&1, fsm.execute(1));
        assert_eq!(&2, fsm.execute(2));
        assert_eq!(&3, fsm.execute(3));
    }

    #[test]
    fn test_string() {
        let mut fsm = StateMachine::<String, String>::new();
        fsm.add_states(&mut vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
        ]);
        fsm.add_transition("alpha".to_string(), "beta".to_string(), |_, _| {
            "beta".to_string()
        });
        println!("{:?}", fsm);
    }
}
