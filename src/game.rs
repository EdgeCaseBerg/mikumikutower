use crate::renderer::Color;
use crate::renderer::Renderer;
use crate::scene::Scene;

use std::time::Duration;
use std::time::Instant;

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
    pub scene: Option<Box<dyn Scene>>,
}

pub struct GameContext {
    pub renderer: Option<Box<dyn Renderer>>,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext { renderer: None }
    }
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
            scene: None,
        }
    }

    pub fn update(&mut self, game_context: &mut GameContext) {
        // For now let's just do 60hz, we can swap this to vsync mode later on in life.
        // https://gameprogrammingpatterns.com/game-loop.html#stuck-in-the-middle
        let ns_per_update = 1_000_000_000 / 60;
        // completely arbitrary but would control how much lag is acceptable
        let max_loops_per_update = 10;
        let current = self.start_time.elapsed().as_nanos();
        let elapsed = current - self.prev_tick;
        self.prev_tick = current;
        self.next_tick = current + ns_per_update;
        self.lag += elapsed;
        self.tick_loops = 0;
        // TODO: add max loop counter here to bail out to avoid system lags
        while self.lag >= ns_per_update && self.tick_loops < max_loops_per_update {
            self.lag -= ns_per_update;
            self.tick_loops += 1;
        }
        if self.tick_loops == max_loops_per_update {
            // TODO prop log library here.
            eprintln!("Frame skipping detected, attempting to correct by skipping frames ahead");
            self.prev_tick = self.start_time.elapsed().as_nanos();
            self.next_tick = self.prev_tick + ns_per_update;
            self.tick_loops = 0;
        }
        // Then this is where we'd call .update per game entity within the current screen/scene
        // and pass along the lag and call the loop as many times as needed. Potentially we would
        // also move the coordinates backing drawing along but we'll see about that.
        // println!(
        //     "Frame time: {:?} {:?}",
        //     self.lag,
        //     self.lag as f32 / ns_per_update as f32
        // );
        // println!(
        //     "elapsed: {:?} prev: {}, next: {}, time_is_not_warped: {} loops {}",
        //     elapsed,
        //     self.prev_tick,
        //     self.next_tick,
        //     self.prev_tick < self.next_tick,
        //     self.tick_loops
        // );

        let scene = self.scene.as_mut();
        if let Some(scene) = scene {
            scene.update(self.tick_loops, game_context);
        }
    }

    pub fn draw(&mut self, game_context: &mut GameContext) {
        self.should_draw = false;
        // Since game update ticks are independent from render ticks
        // we need to compute the proper amount of lag time for what
        // to show to the user.

        // TODO: Move to shared state assuming we'd update game and render per update the same.
        let ns_per_update = 1_000_000_000 / 60;

        let current = self.start_time.elapsed().as_nanos();

        // If we're already past the time to draw, then align the next tick to the current time
        if current > self.next_draw_tick + ns_per_update {
            self.next_draw_tick = current;
        }

        // if we're not past the time to draw, advance by update rate until it's time to render.
        while self.start_time.elapsed().as_nanos() >= self.next_draw_tick {
            self.next_draw_tick += ns_per_update;
            self.should_draw = true;
        }

        if !self.should_draw {
            // Arbitrary constant for now.
            ::std::thread::sleep(Duration::from_millis(2));
            return;
        }

        // This seems very silly and annoying to have to do this twice
        {
            let renderer = game_context.renderer.as_mut();
            if let Some(renderer) = renderer {
                renderer.clear(Color::black());
            }
        }

        let scene = self.scene.as_mut();
        if let Some(scene) = scene {
            scene.draw(game_context);
        }

        {
            let renderer = game_context.renderer.as_mut();
            if let Some(renderer) = renderer {
                renderer.present();
            }
        }
    }
}
