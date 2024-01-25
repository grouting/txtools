use crate::utils::{get_boolean_input, pick_file, pick_files, save_image_with_extension_override};
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use std::io;

pub fn apply_opacity_mask() -> io::Result<()> {
	let path_to_mask = pick_file("Select mask");
	let mask = ImageReader::open(path_to_mask.clone())?
		.decode()
		.expect("could not decode mask");

	let files = pick_files("Select the image(s) that you want to apply the mask to");

	let overwrite_image = get_boolean_input("overwrite image(s)? (y/n)")?;

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

		save_image_with_extension_override(
			new_image,
			path.clone(),
			overwrite_image,
			"masked",
			"png",
		);
	}

	println!("done");

	Ok(())
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
