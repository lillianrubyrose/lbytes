mod macros;

use std::{
	io::{Read, Seek, Write},
	string::FromUtf8Error,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),
	#[error("Utf8: {0}")]
	FromUtf8(#[from] FromUtf8Error),
}

pub trait BytesReadExt: Read + Seek {
	define_integral_r!(i8, 1);
	define_integral_r!(u8, 1);

	define_integral_r!(i16, 2);
	define_integral_r!(u16, 2);

	define_integral_r!(i32, 4);
	define_integral_r!(u32, 4);

	define_integral_r!(i64, 8);
	define_integral_r!(u64, 8);

	fn read_to_vec(&mut self) -> Result<Vec<u8>, Error> {
		let mut buf = Vec::new();
		self.read_to_end(&mut buf)?;
		Ok(buf)
	}

	fn read_n_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error> {
		let mut bytes = [0u8; N];
		self.read_exact(&mut bytes)?;
		Ok(bytes)
	}

	fn read_n_bytes_vec(&mut self, amount: usize) -> Result<Vec<u8>, Error> {
		let mut bytes = vec![0; amount];
		self.read_exact(&mut bytes)?;
		Ok(bytes)
	}

	fn read_f32(&mut self) -> Result<f32, Error> {
		let v = self.read_u32()?;
		Ok(f32::from_bits(v))
	}

	fn read_f64(&mut self) -> Result<f64, Error> {
		let v = self.read_u64()?;
		Ok(f64::from_bits(v))
	}

	fn read_string(&mut self) -> Result<String, Error> {
		let len = self.read_u64()?;
		Ok(String::from_utf8(self.read_n_bytes_vec(len as usize)?)?)
	}
}

pub trait BytesWriteExt: Write {
	define_write!(i8);
	define_write!(u8);

	define_write!(i16);
	define_write!(u16);

	define_write!(i32);
	define_write!(u32);

	define_write!(i64);
	define_write!(u64);

	define_write!(f32);
	define_write!(f64);

	fn write_string<S: AsRef<str>>(&mut self, value: S) -> Result<(), Error> {
		let value = value.as_ref();
		self.write_u64(value.len() as u64)?;
		self.write_all(value.as_bytes())?;
		Ok(())
	}
}

impl<R: Read + Seek> BytesReadExt for R {}
impl<R: Write> BytesWriteExt for R {}

#[cfg(test)]
mod tests {
	use std::io::Cursor;

	use super::*;

	#[test]
	fn string() {
		const CONTENTS: &str = "Hello World. Trans Rights 🏳️‍⚧️ 🏳️‍⚧️ 🏳️‍⚧️ 🦀 🦀 🦀";
		let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
		buffer.write_string(CONTENTS.to_string()).expect("Error when writing");
		buffer.set_position(0);
		assert_eq!(CONTENTS, buffer.read_string().expect("Error when reading"));
	}

	macro_rules! define_test {
		($ty:ty) => {
			paste::item! {
				#[test]
				fn [<$ty>]() {
					const VALUES: [$ty; 2] = [$ty::MAX, $ty::MIN];

					for v in VALUES {
						let mut buffer: Cursor<Vec<u8>> =
							Cursor::new(Vec::new());
						buffer.[<write_$ty>](v).expect("Error when writing");
						buffer.set_position(0);
						assert_eq!(
							v,
							buffer.[<read_$ty>]().expect("Error when reading")
						);
					}
				}
			}
		};
	}

	define_test!(i8);
	define_test!(u8);
	define_test!(i16);
	define_test!(u16);
	define_test!(i32);
	define_test!(u32);
	define_test!(i64);
	define_test!(u64);
	define_test!(f32);
	define_test!(f64);
}
