use std::fs;
use std::io::prelude::*;

pub struct Cartridge {
	pub buffer: [u8; 0xE00],
	pub size: usize
}

impl Cartridge {
	pub fn new(filename: &str) -> Cartridge {
		let mut buffer = [0; 0xE00];
		let mut file = fs::File::open(filename).
			expect("Could not open file");

		let size = file.read(&mut buffer).unwrap();

		Cartridge {
			buffer,
			size
		}
	}
}