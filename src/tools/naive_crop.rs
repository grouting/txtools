use crate::utils::{get_boolean_input, get_input, pick_files, save_image};
use image::io::Reader as ImageReader;
use std::io;

pub fn naive_crop() -> io::Result<()> {
	let files = pick_files("Select the file(s) you want to crop");
	let overwrite = get_boolean_input("overwrite image(s)? (y/n)")?;
	let centre = get_boolean_input("centre image(s) before cropping? (y/n)")?;

	if files.len() == 1 {
		let image = ImageReader::open(files.first().unwrap().clone())?
			.decode()
			.expect("could not decode image");

		println!(
			"selected image's dimensions: width: {}, height: {}",
			image.width(),
			image.height()
		);
	}

	println!("specify target width:");
	let target_width = get_input()?.parse::<u32>().expect("invalid width");
	println!("specify target height:");
	let target_height = get_input()?.parse::<u32>().expect("invalid height");

	for path in files {
		let mut image = ImageReader::open(path.clone())?
			.decode()
			.expect("could not decode image");

		if image.width() < target_width || image.height() < target_height {
			println!(
				"{:?} has invalid dimensions; skipping",
				path.file_name().unwrap()
			);
			continue;
		}

		let x = if centre {
			(image.width() - target_width) / 2
		} else {
			0
		};

		let y = if centre {
			(image.height() - target_height) / 2
		} else {
			0
		};

		let new_image = image.crop(x, y, target_width, target_height);

		save_image(new_image, path.clone(), overwrite, "cropped");
	}

	println!("done");

	Ok(())
}
