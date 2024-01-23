use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use rfd::FileDialog;
use std::{
	io,
	path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
	let mut main_menu = Menu::new();
	main_menu.add_choice(Choice::new("1", "Pixelate"));
	main_menu.add_choice(Choice::new("2", "Apply Opacity Mask"));

	println!("Select tool:");
	main_menu.print_choices();

	let selection = main_menu.get_choice_input()?;

	match selection.id.as_str() {
		"1" => pixelate(),
		"2" => apply_opacity_mask(),
		_ => {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"invalid choice",
			));
		}
	}
}

struct Menu {
	choices: Vec<Choice>,
}

impl Menu {
	fn new() -> Menu {
		Menu { choices: vec![] }
	}

	fn add_choice(&mut self, choice: Choice) {
		self.choices.push(choice);
	}

	fn print_choices(&self) {
		for choice in self.choices.iter() {
			choice.print();
		}
	}

	fn get_choice_input(&self) -> io::Result<&Choice> {
		let input = get_input()?;

		for choice in self.choices.iter() {
			if choice.id == input {
				return Ok(choice);
			}
		}

		Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"invalid choice",
		))
	}
}

struct Choice {
	id: String,
	label: String,
}

impl Choice {
	fn new(id: &str, label: &str) -> Choice {
		Choice {
			id: String::from(id),
			label: String::from(label),
		}
	}

	fn print(&self) {
		println!("{}: {}", self.id, self.label);
	}
}

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;

	let length = buffer.trim_end_matches(&['\r', '\n'][..]).len();
	buffer.truncate(length);

	Ok(buffer)
}

fn pick_file(title: &str) -> PathBuf {
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

fn pick_files(title: &str) -> Vec<PathBuf> {
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

fn pixelate() -> io::Result<()> {
	let files = pick_files("Select the image(s) you want to pixelate");

	println!("specify scale:");
	let scale = get_input()?.parse::<u32>().expect("invalid scale");

	if scale < 2 || scale > 16 {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"scale must be between 2 and 16",
		));
	}

	println!("overwrite image(s)? (y/n)");
	let overwrite_image = match get_input()?.as_str() {
		"y" | "Y" => Ok(true),
		"n" | "N" => Ok(false),
		_ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
	}?;

	for path in files {
		let image = ImageReader::open(path.clone())?
			.decode()
			.expect("could not decode image");

		if image.width() % scale != 0 || image.height() % scale != 0 {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"image width must be divisible by scale",
			));
		}

		let mut new_image = DynamicImage::new_rgb8(image.width() / scale, image.height() / scale);

		for x in 0..new_image.width() {
			for y in 0..new_image.height() {
				let mut pixels: Vec<Rgba<u8>> = vec![Rgba([0, 0, 0, 0]); (scale * scale) as usize];

				for i in 0..scale {
					for j in 0..scale {
						pixels[(i * scale + j) as usize] = image.get_pixel(
							x * (scale as u32) + i as u32,
							y * (scale as u32) + j as u32,
						);
					}
				}

				let pixel = average_pixels(&pixels);

				new_image.put_pixel(x, y, pixel);
			}
		}

		let directory = &path
			.ancestors()
			.nth(1)
			.unwrap_or_else(|| Path::new("."))
			.display();

		let original_file_name = &path.file_name().unwrap().to_str().unwrap();

		let file_name = if overwrite_image {
			original_file_name.to_string()
		} else {
			format!("pixelated_{}", original_file_name)
		};

		new_image
			.save(format!("{}/{}", directory, file_name))
			.expect("could not save image");

		println!("completed processing {}", original_file_name);
	}

	println!("done");

	Ok(())
}

fn average_pixels(pixels: &Vec<Rgba<u8>>) -> Rgba<u8> {
	let mut red: u32 = 0;
	let mut green: u32 = 0;
	let mut blue: u32 = 0;
	let mut alpha: u32 = 0;

	for pixel in pixels {
		red += pixel[0] as u32;
		green += pixel[1] as u32;
		blue += pixel[2] as u32;
		alpha += pixel[3] as u32;
	}

	let pixel_count = pixels.len() as u32;

	Rgba([
		(red / pixel_count) as u8,
		(green / pixel_count) as u8,
		(blue / pixel_count) as u8,
		(alpha / pixel_count) as u8,
	])
}

fn apply_opacity_mask() -> io::Result<()> {
	let path_to_mask = pick_file("Select mask");
	let mask = ImageReader::open(path_to_mask.clone())?
		.decode()
		.expect("could not decode mask");

	let files = pick_files("Select the image(s) that you want to apply the mask to");

	println!("overwrite image(s)? (y/n)");
	let overwrite_image = match get_input()?.as_str() {
		"y" | "Y" => Ok(true),
		"n" | "N" => Ok(false),
		_ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
	}?;

	for path in files {
		let image = ImageReader::open(path.clone())?
			.decode()
			.expect("could not decode image");

		if mask.width() != image.width() || mask.height() != image.height() {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"image dimensions must match mask dimensions",
			));
		}

		let mut new_image = DynamicImage::new_rgba8(image.width(), image.height());

		for x in 0..image.width() {
			for y in 0..image.height() {
				new_image.put_pixel(
					x,
					y,
					get_new_pixel(mask.get_pixel(x, y), image.get_pixel(x, y)),
				);
			}
		}

		let directory = &path
			.ancestors()
			.nth(1)
			.unwrap_or_else(|| Path::new("."))
			.display();

		let original_file_name = &path.file_name().unwrap().to_str().unwrap();

		let extension_index = original_file_name.rfind('.').unwrap();

		let new_file_name: String = original_file_name.chars().take(extension_index).collect();

		let file_name = if overwrite_image {
			new_file_name.to_string()
		} else {
			format!("masked_{}", new_file_name)
		};

		new_image
			.save(format!("{}/{}.png", directory, file_name))
			.expect("could not save image");

		println!("completed processing {}", original_file_name);
	}

	fn get_new_pixel(mask: Rgba<u8>, texture: Rgba<u8>) -> Rgba<u8> {
		let mask = mask.channels();
		let texture = texture.channels();

		Rgba::<u8>([
			*texture.get(0).unwrap(),
			*texture.get(1).unwrap(),
			*texture.get(2).unwrap(),
			*mask.get(0).unwrap(),
		])
	}

	println!("done");

	Ok(())
}
