pub struct GameOptions {
	pub window_width: usize,
	pub window_height: usize,
	// Eventually we can add log level when we add a real logger in
}

impl Default for GameOptions {
	fn default() -> Self {
		Self{
			window_width: 640,
			window_height: 480
		}
	}
}