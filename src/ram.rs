use std::io::Read;
#[derive(Debug)]
pub struct Error(pub String);
pub struct Ram {
	pub buf: [u8; 0xFFF], //0xFFF = 4096
}

impl Ram {
	pub fn new() -> Self {
		Ram { buf: [0; 0xFFF] }
	}

	/// Load Chip8 ROM into memory.
	pub fn load<S: Read>(&mut self, mut stream: S) -> std::io::Result<()> {
		self.load_fontset();
		loop {
			if stream.read(&mut self.buf[0x200..])? == 0 {
				break;
			}
		}
		println!("finish");
		Ok(())
	}

	fn load_fontset(&mut self) {
		let fontset = vec![
			0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
			0x20, 0x60, 0x20, 0x20, 0x70, // 1
			0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
			0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
			0x90, 0x90, 0xF0, 0x10, 0x10, // 4
			0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
			0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
			0xF0, 0x10, 0x20, 0x40, 0x40, // 7
			0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
			0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
			0xF0, 0x90, 0xF0, 0x90, 0x90, // A
			0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
			0xF0, 0x80, 0x80, 0x80, 0xF0, // C
			0xE0, 0x90, 0x90, 0x90, 0xE0, // D
			0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
			0xF0, 0x80, 0xF0, 0x80, 0x80, // F
		];
		&self.buf[..fontset.len()].copy_from_slice(&fontset);
	}
}
