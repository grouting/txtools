use rfd::FileDialog;
use std::{io, path::PathBuf};

pub fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;

	let length = buffer.trim_end_matches(&['\r', '\n'][..]).len();
	buffer.truncate(length);

	Ok(buffer)
}

pub fn pick_file(title: &str) -> PathBuf {
	FileDialog::new()
		.add_filter(
			"image",
			&["png", "jpg", "jpeg", "bmp", "avif", "tga", "tiff", "webp"],
		)
		.set_directory("/")
		.set_title(title)
		.pick_file()
		.expect("no file selected")
}

pub fn pick_files(title: &str) -> Vec<PathBuf> {
	FileDialog::new()
		.add_filter(
			"image",
			&["png", "jpg", "jpeg", "bmp", "avif", "tga", "tiff", "webp"],
		)
		.set_directory("/")
		.set_title(title)
		.pick_files()
		.expect("no files selected")
}
