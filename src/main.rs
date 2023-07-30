mod emulator;
mod ui;

use ui::App;

const WINDOW_WIDTH: f32 = 980.0;
const WINDOW_HEIGHT: f32 = 525.0;

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
		default_theme: eframe::Theme::Light,
		always_on_top: true,
		resizable: true,
		..Default::default()
	};

	eframe::run_native(
		"Chip-8 Emulator",
		options,
		Box::new(|_cc| Box::<App>::default()),
	)
}
