use std::collections::HashMap;

use sdl2::ttf::{Sdl2TtfContext, Font};

pub struct StringFonts {
	pub electrolize: String,
}

pub struct Fonts {
	pub electrolize:  &'static str,
}

impl Fonts {
	pub fn to_strings(&self) -> StringFonts {
		StringFonts {
			electrolize: self.electrolize.to_string(),
		}
	}
}

pub static FONTS: Fonts = Fonts {
	electrolize: "Electrolize/Electrolize-Regular.ttf",
};

pub fn load_fonts(binary_filepath: String, ttf_context: &Sdl2TtfContext) -> HashMap<String, Font> {
	let mut fonts = HashMap::new();
	let string_fonts = FONTS.to_strings();

	// Load a font
	// let mut font = ttf_context.load_font(font_path, 128)?;
	// font.set_style(sdl2::ttf::FontStyle::BOLD);

	fonts.insert(string_fonts.electrolize.to_string(), ttf_context.load_font(binary_filepath.clone() + &string_fonts.electrolize, 128).unwrap());

	fonts
}