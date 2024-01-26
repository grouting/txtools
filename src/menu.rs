use std::io;

pub struct Menu<F>
where
	F: Fn() -> io::Result<()>,
{
	choices: Vec<Choice<F>>,
}

#[allow(clippy::new_without_default)]
impl<F> Menu<F>
where
	F: Fn() -> io::Result<()>,
{
	pub fn new() -> Menu<F> {
		Menu { choices: vec![] }
	}

	pub fn add_choice(&mut self, choice: Choice<F>)
	where
		F: Fn() -> io::Result<()>,
	{
		self.choices.push(choice);
	}

	pub fn print_choices(&self) {
		for choice in self.choices.iter() {
			choice.print();
		}
	}

	fn get_choice_input(&self) -> io::Result<&Choice<F>>
	where
		F: Fn() -> io::Result<()>,
	{
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

	pub fn execute_choice_input(&self) -> io::Result<()> {
		let choice = self.get_choice_input()?;
		choice.execute()
	}
}

pub struct Choice<F>
where
	F: Fn() -> io::Result<()>,
{
	id: String,
	label: String,
	method: F,
}

impl<F> Choice<F>
where
	F: Fn() -> io::Result<()>,
{
	pub fn new(id: &str, label: &str, method: F) -> Choice<F> {
		Choice {
			id: String::from(id),
			label: String::from(label),
			method,
		}
	}

	fn print(&self) {
		println!("{}: {}", self.id, self.label);
	}

	pub fn execute(&self) -> io::Result<()> {
		(self.method)()
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
