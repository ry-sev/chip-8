use egui::{containers::Frame, Color32, Margin, Rounding};
use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
	static ref BASE_FRAME: Frame = Frame {
		outer_margin: Margin { left: 10., right: 10., top: 10., bottom: 10., },
		rounding: Rounding { nw: 10., ne: 10., sw: 10., se: 10., },
		..Default::default()
	};
}

#[derive(Debug, PartialEq, Eq)]
pub enum Theme {
	Retro,
	Hacker,
	Dark,
	Light,
	Red,
}

impl Theme {
	pub fn foreground(&self) -> (u8, u8, u8) {
		match self {
			Self::Retro => (255, 176, 0),
			Self::Hacker => (50, 255, 102),
			Self::Dark => (255, 255, 255),
			Self::Light => (0, 0, 0),
			Self::Red => (233, 76, 61),
		}
	}
	pub fn frame(&self) -> Frame {
		match self {
			Self::Retro => BASE_FRAME.fill(Color32::from_rgb(33, 20, 5)),
			Self::Hacker => BASE_FRAME.fill(Color32::from_rgb(6, 21, 14)),
			Self::Dark => BASE_FRAME.fill(Color32::BLACK),
			Self::Light => BASE_FRAME.fill(Color32::WHITE),
			Self::Red => BASE_FRAME.fill(Color32::from_rgb(45, 41, 38)),
		}
	}
}
