extern crate alloc;

use alloc::vec::Vec;
use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse, ASCII};

const LOOP_COUNT: usize = 10;

fn main() {
    let throbber = Throbber::default()
        .throbber_set(ASCII)
        .use_type(WhichUse::Spin);

    let mut states: Vec<ThrobberState> = Vec::new();
    let mut state = ThrobberState::default();

    // Basic operations
    state.calc_next();
    state.calc_step(2);
    state.calc_step(-1);
    state.calc_step(0); // no change in no-std
    state.normalize(&throbber);
    states.push(state.clone());

    // Simulate animation loop
    for _ in 0..LOOP_COUNT {
        state.calc_next();
        let mut normalized = state.clone();
        normalized.normalize(&throbber);
        states.push(normalized);
    }

    // Check final state
    let expected_state_index = 2 + LOOP_COUNT as i8;
    let expected_last_normalized_index = match LOOP_COUNT % 4 {
        1 => 3,
        2 => 0,
        3 => 1,
        0 => 2,
        _ => unreachable!(),
    };
    let expected_states_len = 1 + LOOP_COUNT;

    if state.index() != expected_state_index
        || states.last().unwrap().index() != expected_last_normalized_index
        || states.len() != expected_states_len
    {
        panic!("State mismatch");
    }
}
