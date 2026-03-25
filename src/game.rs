use std::time::Instant;

#[derive(Debug)]
pub struct Game {
    start_time: Instant,
    // counters to track game updates on a fixed interval with catch up
    prev_tick: u128,
    next_tick: u128,
    pub tick_loops: u32,
    lag: u128,
    // counters to track if we should render anything when drawing (variable rendering)
    next_draw_tick: u128,
    should_draw: bool,
    // TODO: current scene and state when we make em
}

impl Game {
    pub fn new() -> Self {
        Game {
            start_time: Instant::now(),
            prev_tick: 0,
            next_tick: 0,
            tick_loops: 0,
            lag: 0,
            next_draw_tick: 0,
            should_draw: false,
        }
    }

    pub fn update(&mut self) {
        // For now let's just do 60hz, we can swap this to vsync mode later on in life.
        // https://gameprogrammingpatterns.com/game-loop.html#stuck-in-the-middle
        let ns_per_update = 1_000_000_000 / 60;
        let current = self.start_time.elapsed().as_nanos();
        let elapsed = current - self.prev_tick;
        self.prev_tick = current;
        self.next_tick = current + ns_per_update;
        self.lag += elapsed;
        self.tick_loops = 0;
        // TODO: add max loop counter here to bail out to avoid system lags
        while self.lag >= ns_per_update {
            self.lag -= ns_per_update;
            self.tick_loops += 1;
        }
        println!(
            "Frame time: {:?} {:?}",
            self.lag,
            self.lag as f32 / ns_per_update as f32
        );
        println!(
            "elapsed: {:?} prev: {}, next: {}, time_is_not_warped: {} loops {}",
            elapsed,
            self.prev_tick,
            self.next_tick,
            self.prev_tick < self.next_tick,
            self.tick_loops
        );
        // now we're ready to render
    }
}
