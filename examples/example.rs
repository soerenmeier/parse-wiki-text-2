use std::{fs, io};
use io::Read;
use clap::Parser;

use parse_wiki_text_2::Configuration;


#[derive(Parser)]
pub struct Args {
	/// Path to a file to parse.
	///
	/// If no file is specified Stdin is used
	file: Option<String>
}

fn main() {
	let args = Args::parse();

	match args.file {
		Some(file) => {
			let Ok(content) = fs::read_to_string(&file) else {
				panic!("Failed to read file {file}");
			};

			let parsed = Configuration::default().parse(&content);
			println!("{parsed:#?}");
		},
		None => {
			// read stdin
			let mut stdin = io::stdin();
			let mut content = String::new();
			stdin.read_to_string(&mut content)
				.expect("could not read stdin as string");

			let parsed = Configuration::default().parse(&content);
			println!("{parsed:#?}");
		}
	}
}