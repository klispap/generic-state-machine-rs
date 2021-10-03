#[cfg(test)]
mod static_sanity_test {
    use crate::error::Result;
    use crate::state::State;
    use crate::event::Event;
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
}

mod dynamic_sanity_test {
    use crate::error::Result;
    use crate::state::State;
    use crate::event::Event;
    use crate::dynamic_state_machine::DynamicStateMachine as StateMachine;
    use crate::dynamic_state_machine::DynamicTransition as Transition;
    
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
