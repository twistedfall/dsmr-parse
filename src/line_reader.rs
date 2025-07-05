use std::io;

pub struct LineReader<B> {
	inner: B,
}

impl<B: Iterator<Item = io::Result<u8>>> LineReader<B> {
	pub fn new(inner: B) -> Self {
		Self { inner }
	}
}

impl<B: Iterator<Item = io::Result<u8>>> Iterator for LineReader<B> {
	type Item = io::Result<Vec<u8>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut line = Vec::with_capacity(64);
		loop {
			match self.inner.next()? {
				Ok(b) => {
					if b == b'\n' {
						if line.ends_with(b"\r") {
							line.pop();
						}
						break Some(Ok(line));
					}
					line.push(b);
				}
				Err(e) => break Some(Err(e)),
			}
		}
	}
}
