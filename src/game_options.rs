pub struct GameOptions {
	pub window_width: u32,
	pub window_height: u32,
	// Eventually we can add log level when we add a real logger in
}

impl Default for GameOptions {
	fn default() -> Self {
		Self{
			window_width: 1280,
			window_height: 720
		}
	}
}