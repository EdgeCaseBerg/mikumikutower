use std::time::Instant;

pub struct Game {
    start_time: Instant,
    // counters to track game updates on a fixed interval with catch up
    last_tick: u128,
    next_tick: u128,
    pub tick_loops: u32,
    // counters to track if we should render anything when drawing (variable rendering)
    next_draw_tick: u128,
    should_draw: bool,
    // TODO: current scene and state when we make em
}
