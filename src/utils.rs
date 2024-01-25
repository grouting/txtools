use image::DynamicImage;
use rfd::FileDialog;
use std::{
	io,
	path::{Path, PathBuf},
};

pub fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;

	let length = buffer.trim_end_matches(&['\r', '\n'][..]).len();
	buffer.truncate(length);

	Ok(buffer)
}

pub fn get_boolean_input(prompt: &str) -> io::Result<bool> {
	println!("{}", prompt);

	match get_input()?.as_str() {
		"y" | "Y" => Ok(true),
		"n" | "N" => Ok(false),
		_ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
	}
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

pub fn save_image(new_image: DynamicImage, path: PathBuf, overwrite: bool, prefix: &str) {
	let directory = &path
		.ancestors()
		.nth(1)
		.unwrap_or_else(|| Path::new("."))
		.display();

	let original_file_name = &path.file_name().unwrap().to_str().unwrap();

	let file_name = if overwrite {
		original_file_name.to_string()
	} else {
		format!("{}_{}", prefix, original_file_name)
	};

	if let Err(_) = new_image.save(format!("{}/{}", directory, file_name)) {
		println!("could not save {}", original_file_name);
	} else {
		println!("finished processing {}", original_file_name);
	};
}

pub fn save_image_with_extension_override(
	new_image: DynamicImage,
	path: PathBuf,
	overwrite: bool,
	prefix: &str,
	extension: &str,
) {
	let directory = &path
		.ancestors()
		.nth(1)
		.unwrap_or_else(|| Path::new("."))
		.display();

	let original_file_name = &path.file_name().unwrap().to_str().unwrap();

	let extension_index = original_file_name.rfind('.').unwrap();
	let new_file_name: String = original_file_name.chars().take(extension_index).collect();

	let file_name = if overwrite {
		new_file_name.to_string()
	} else {
		format!("{}_{}", prefix, new_file_name)
	};

	if let Err(_) = new_image.save(format!("{}/{}.{}", directory, file_name, extension)) {
		println!("could not save {}", original_file_name);
	} else {
		println!("finished processing {}", original_file_name);
	};
}
