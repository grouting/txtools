use std::io;
use txtool::{tools, Choice, Menu};

fn main() -> io::Result<()> {
	let mut main_menu = Menu::new();
	main_menu.add_choice(Choice::new(
		"1",
		"Pixelate",
		tools::pixelate as fn() -> io::Result<()>,
	));
	main_menu.add_choice(Choice::new(
		"2",
		"Apply Opacity Mask",
		tools::apply_opacity_mask as fn() -> io::Result<()>,
	));

	println!("Select tool:");
	main_menu.print_choices();

	main_menu.execute_choice_input()
}
