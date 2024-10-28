use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to initialize fontconfig")]
	FontconfigInit,
}

pub struct FontFinder {
	#[cfg(feature = "fontconfig")]
	fontconfig: fontconfig::Fontconfig,
}

impl FontFinder {
	pub fn new() -> Result<Self, Error> {
		#[cfg(feature = "fontconfig")]
		{
			let fontconfig =
				fontconfig::Fontconfig::new().ok_or(Error::FontconfigInit)?;
			Ok(Self { fontconfig })
		}
	}

	pub fn find_font(
		&self,
		family: &str,
		style: Option<&str>,
	) -> Option<PathBuf> {
		#[cfg(feature = "fontconfig")]
		{
			self.fontconfig.find(family, style).map(|font| font.path)
		}

		#[cfg(not(feature = "fontconfig"))]
		{
			self.find_font_fallback(family, style)
		}
	}

	#[cfg(not(feature = "fontconfig"))]
	fn find_font_fallback(
		&self,
		family: &str,
		style: Option<&str>,
	) -> Option<PathBuf> {
		use walkdir::WalkDir;

		let font_dirs = [
			"/usr/share/fonts",
			"/usr/local/share/fonts",
			"/usr/share/fonts/truetype",
			"/usr/local/share/fonts/truetype",
			"/system/fonts",
		];

		for font_directory in font_dirs.iter() {
			for entry in WalkDir::new(font_directory) {
				let entry = entry.unwrap();

				if entry.file_type().is_file()
					& (entry.path().extension() == Some("ttf".as_ref())
						|| entry.path().extension() == Some("otf".as_ref()))
				{
					if let Some(file_name) = entry.file_name().to_str() {
						if file_name.starts_with(family) {
							if let Some(style) = style {
								if file_name.contains(style) {
									return Some(entry.path().to_path_buf());
								}
							} else {
								return Some(entry.path().to_path_buf());
							}
						}
					}
				}
			}
		}

		None
	}
}
