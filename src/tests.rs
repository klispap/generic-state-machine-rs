#[cfg(test)]
mod primitive_tests {
    use crate::primitives::StateMachine;
    /// Test over the following state machine:
    ///            +---->[1]----+
    ///   Event: 1 |            | Event: 2
    ///            |            V
    ///           [3]          [2]
    ///            ^            |
    ///            |            | Event: 3
    ///            +------------+
    fn tf(_fsm: &StateMachine<i32, i32>, event: i32) -> i32 {
        match event {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => panic!(), // Invalid transition
        }
    }
    #[test]
    fn primitive_test_i32() {
        let mut fsm = StateMachine::<i32, i32>::new();
        fsm.add_states(&[1, 2, 3])
            .add_transition(1, 2, |_fsm, _event| 2)
            .add_transition(2, 3, tf)
            .add_transition(3, 1, tf)
            .initial_state(1)
            .unwrap();

        println!("fsm: {:?}", fsm);

        assert_eq!(2, fsm.execute(2).unwrap());
        assert_eq!(3, fsm.execute(3).unwrap());
        assert_eq!(1, fsm.execute(1).unwrap());
    }

    #[test]
    fn primitive_test_string() {
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

#[cfg(all(test, feature = "async"))]
mod state_machine_tests {
    use crate::state_machine::AsyncStateMachine;

    #[tokio::test]
    async fn sanity_test() {
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

        println!("State Machine under test: {:?}", fsm);

        fsm.start().await.unwrap();

        println!("State Machine under test: {:?}", fsm);

        fsm.push_event(0).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        assert_eq!(fsm.current_state().await, false);

        fsm.push_event(1).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        assert_eq!(fsm.current_state().await, true);

        fsm.push_event(1).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        assert_eq!(fsm.current_state().await, true);

        fsm.push_event(0).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        assert_eq!(fsm.current_state().await, false);

        fsm.push_event(0).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        assert_eq!(fsm.current_state().await, false);
    }
}
