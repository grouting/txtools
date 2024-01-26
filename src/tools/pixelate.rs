use crate::utils::{get_boolean_input, get_input, pick_files, save_image};
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::io;

pub fn pixelate() -> io::Result<()> {
	let files = pick_files("Select the image(s) you want to pixelate");

	println!("specify scale:");
	let scale = get_input()?.parse::<u32>().expect("invalid scale");

	if !(2..=16).contains(&scale) {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"scale must be between 2 and 16",
		));
	}

	let overwrite_image = get_boolean_input("overwrite image(s)? (y/n)")?;

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
						pixels[(i * scale + j) as usize] =
							image.get_pixel(x * (scale as u32) + i, y * (scale as u32) + j);
					}
				}

				let pixel = average_pixels(&pixels);

				new_image.put_pixel(x, y, pixel);
			}
		}

		save_image(new_image, path.clone(), overwrite_image, "pixelated")
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
