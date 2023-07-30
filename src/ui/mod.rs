mod themes;

use crate::emulator::{
	Emulator, DISPLAY_HEIGHT, DISPLAY_WIDTH, SCALE, TICKS_PER_FRAME,
};
use anyhow::Result;
use egui::{Color32, ColorImage};
use egui_extras::RetainedImage;
use std::path::PathBuf;
use themes::Theme;

fn get_roms() -> Result<Vec<PathBuf>> {
	let roms = std::fs::read_dir("roms")?
		.filter_map(|res| res.ok())
		.map(|dir_entry| dir_entry.path())
		.filter_map(|path| {
			if path.extension().map_or(false, |ext| ext == "ch8") {
				Some(path)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	Ok(roms)
}

pub struct App {
	theme: Theme,
	roms: Vec<PathBuf>,
	emulator: Emulator,
}

impl App {
	pub fn new() -> Self {
		Self {
			theme: Theme::Hacker,
			roms: get_roms().unwrap(),
			emulator: Emulator::default(),
		}
	}

	fn handle_key_event(&mut self, ui: &mut egui::Ui) {
		use egui::Key;

		for (key, byte) in [
			(Key::Num1, 0x1),
			(Key::Num2, 0x2),
			(Key::Num3, 0x3),
			(Key::Num4, 0xC),
			(Key::Q, 0x4),
			(Key::W, 0x5),
			(Key::E, 0x6),
			(Key::R, 0xD),
			(Key::A, 0x7),
			(Key::S, 0x8),
			(Key::D, 0x9),
			(Key::F, 0xE),
			(Key::Z, 0xA),
			(Key::X, 0x0),
			(Key::C, 0xB),
			(Key::V, 0xF),
		] {
			ui.input_mut(|i| {
				if i.key_down(key) {
					self.emulator.press_key(byte, true);
				} else {
					self.emulator.press_key(byte, false);
				}
			});
		}
	}
}

impl Default for App {
	fn default() -> Self {
		Self::new()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		if self.emulator.running {
			for _ in 0..TICKS_PER_FRAME {
				if let Err(e) = self.emulator.cpu.step() {
					panic!("{e}");
				}
			}
			self.emulator.tick_timer();
			ctx.request_repaint();
		}

		egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("theme", |ui| {
					if ui.button("Retro").clicked() {
						self.theme = Theme::Retro;
					}
					if ui.button("Hacker").clicked() {
						self.theme = Theme::Hacker;
					}
					if ui.button("Dark").clicked() {
						self.theme = Theme::Dark;
					}
					if ui.button("Light").clicked() {
						self.theme = Theme::Light;
					}
					if ui.button("Red").clicked() {
						self.theme = Theme::Red;
					}
				});
				ui.menu_button("roms", |ui| {
					for rom_file in &self.roms {
						let filename =
							rom_file.file_name().unwrap().to_str().unwrap();
						if ui.button(filename).clicked() {
							if let Err(e) = self.emulator.load_rom(rom_file) {
								println!("{}", e);
							}
						}
					}
				});
				if ui.button("quit").clicked() {
					frame.close();
				}
				egui::warn_if_debug_build(ui);
			});
		});

		egui::CentralPanel::default()
			.frame(self.theme.frame())
			.show(ctx, |ui| {
				self.handle_key_event(ui);

				let mut image = ColorImage::new(
					[DISPLAY_WIDTH, DISPLAY_HEIGHT],
					Color32::TRANSPARENT,
				);
				image.pixels = self
					.emulator
					.cpu
					.display
					.chunks(1)
					.map(|p| {
						let (r, g, b) = self.theme.foreground();
						Color32::from_rgba_unmultiplied(r, g, b, p[0] as u8 * 255)
					})
					.collect();

				let retained_image =
					RetainedImage::from_color_image("display.png", image)
						.with_options(egui::TextureOptions::NEAREST);

				let size = egui::Vec2::new(
					retained_image.size()[0] as f32 * SCALE,
					retained_image.size()[1] as f32 * SCALE,
				);

				ui.image(retained_image.texture_id(ctx), size);
			});
	}
}
