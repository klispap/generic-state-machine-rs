#[cfg(test)]
mod static_sanity_test {
    use crate::error::Result;
    use crate::event::Event;
    use crate::state::State;
    use crate::static_state_machine::StateMachine;
    use crate::static_state_machine::Transition;

    use std::sync::Arc;

    #[test]
    fn fsm_sanity_test() -> Result<()> {
        let state1 = Arc::new(State::new(1));
        let state2 = Arc::new(State::new(2));
        let state3 = Arc::new(State::new(3));

        let event_goto_1 = Arc::new(Event::new(1));
        let event_goto_2 = Arc::new(Event::new(2));
        let event_goto_3 = Arc::new(Event::new(3));

        let s = state2.clone();
        let transition12 = Transition::new(*state1, *event_goto_2, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state3.clone();
        let transition23 = Transition::new(*state2, *event_goto_3, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state1.clone();
        let transition31 = Transition::new(*state3, *event_goto_1, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state3.clone();
        let transition13 = Transition::new(*state1, *event_goto_3, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let mut state_machine = StateMachine::new(*state1)
            .insert_transition(transition12)
            .insert_transition(transition13)
            .insert_transition(transition23)
            .insert_transition(transition31);

        let state = state_machine.event(&event_goto_2)?;
        assert_eq!(state, Some(&*state2));

        let state = state_machine.event(&event_goto_3)?;
        assert_eq!(state, Some(&*state3));

        let state = state_machine.event(&event_goto_1)?;
        assert_eq!(state, Some(&*state1));

        let state = state_machine.event(&event_goto_3)?;
        assert_eq!(state, Some(&*state3));

        Ok(())
    }

    use std::sync::mpsc;
    use std::sync::mpsc::{sync_channel, Receiver, Sender};
    use std::{thread, time};

    fn traffic_lights(rx: Receiver<(u8, &str)>) -> () {
        let one_sec = time::Duration::from_secs(1);
        loop {
            let (sleep, color) = rx.recv().expect("Error on recv");

            if sleep == 0 {
                break;
            }

            println!("Traffic light is: {}", color);
            for i in 1..=sleep {
                println!("\tsleep: {}", i);
                thread::sleep(one_sec);
            }
        }
    }

    #[test]
    fn fsm_sanity_test_traffic_lights() -> Result<()> {
        let (tx, rx) = sync_channel(1);

        let state_green = Arc::new(State::new((5u8, "green")));
        let state_yellow = Arc::new(State::new((1u8, "yellow")));
        let state_red = Arc::new(State::new((5u8, "red")));
        let state_red_yellow = Arc::new(State::new((2u8, "red-yellow")));

        let event_green_to_yellow = Arc::new(Event::new(()));
        let event_yellow_to_red = Arc::new(Event::new(()));
        let event_red_to_red_yellow = Arc::new(Event::new(()));
        let event_red_yellow_to_green = Arc::new(Event::new(()));

        let ret_state = state_yellow.clone();
        let transition_gy = Transition::new(*state_green, *event_green_to_yellow, move |c, _| {
            println!("transition: curr: {} - {}", c.content().0, c.content().1);
            Some(*ret_state)
        });

        let ret_state = state_red.clone();
        let transition_yr = Transition::new(*state_yellow, *event_yellow_to_red, move |c, _| {
            println!("transition: curr: {} - {}", c.content().0, c.content().1);
            Some(*ret_state)
        });

        let ret_state = state_red_yellow.clone();
        let transition_ry_g = Transition::new(*state_red, *event_red_to_red_yellow, move |c, _| {
            println!("transition: curr: {} - {}", c.content().0, c.content().1);
            Some(*ret_state)
        });

        let ret_state = state_green.clone();
        let transition_g = Transition::new(
            *state_red_yellow,
            *event_red_yellow_to_green,
            move |c, e| {
                println!("transition: curr: {} - {}", c.content().0, c.content().1);
                Some(*ret_state)
            },
        );

        let mut state_machine = StateMachine::new(*state_green)
            .insert_transition(transition_gy)
            .insert_transition(transition_yr)
            .insert_transition(transition_ry_g)
            .insert_transition(transition_g);

        let t = thread::spawn(|| traffic_lights(rx));

        let state = state_machine.event(&event_green_to_yellow)?.unwrap();
        tx.send((state.content().0, state.content().1));
        assert_eq!(state, &*state_yellow);

        let state = state_machine.event(&event_yellow_to_red)?.unwrap();
        tx.send((state.content().0, state.content().1));
        assert_eq!(state, &*state_red);

        let state = state_machine.event(&event_red_to_red_yellow)?.unwrap();
        tx.send((state.content().0, state.content().1));
        assert_eq!(state, &*state_red_yellow);

        let state = state_machine.event(&event_red_yellow_to_green)?.unwrap();
        tx.send((state.content().0, state.content().1));
        assert_eq!(state, &*state_green);

        tx.send((0u8, ""));
        t.join();

        Ok(())
    }
}

mod dynamic_sanity_test {
    use crate::dynamic_state_machine::DynamicStateMachine as StateMachine;
    use crate::dynamic_state_machine::DynamicTransition as Transition;
    use crate::error::Result;
    use crate::event::Event;
    use crate::state::State;

    use std::sync::Arc;

    #[test]
    fn dynamic_fsm_sanity_test() -> Result<()> {
        let state1 = Arc::new(State::new(1));
        let state2 = Arc::new(State::new(2));
        let state3 = Arc::new(State::new(3));

        let event_goto_1 = Arc::new(Event::new(1));
        let event_goto_2 = Arc::new(Event::new(2));
        let event_goto_3 = Arc::new(Event::new(3));

        let s = state2.clone();
        let transition12 = Transition::new(*state1, *event_goto_2, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state3.clone();
        let transition23 = Transition::new(*state2, *event_goto_3, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state1.clone();
        let transition31 = Transition::new(*state3, *event_goto_1, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let s = state3.clone();
        let transition13 = Transition::new(*state1, *event_goto_3, move |c, e| {
            println!("transition: curr: {}, event: {}", c.content(), e.content());
            Some(*s.clone())
        });

        let mut state_machine = StateMachine::new(*state1)
            .insert_transition(transition12)
            .insert_transition(transition13)
            .insert_transition(transition23)
            .insert_transition(transition31);

        let state = state_machine.event(&event_goto_2)?;
        assert_eq!(state, Some(&*state2));

        let state = state_machine.event(&event_goto_3)?;
        assert_eq!(state, Some(&*state3));

        let state = state_machine.event(&event_goto_1)?;
        assert_eq!(state, Some(&*state1));

        let state = state_machine.event(&event_goto_3)?;
        assert_eq!(state, Some(&*state3));

        Ok(())
    }
}
