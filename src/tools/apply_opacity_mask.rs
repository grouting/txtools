use crate::utils::{get_input, pick_file, pick_files};
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use std::{io, path::Path};

pub fn apply_opacity_mask() -> io::Result<()> {
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
