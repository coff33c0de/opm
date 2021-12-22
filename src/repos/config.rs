use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::errors::ConfigError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub root: String,
	pub cache: String,
	pub info: String,
	pub rls: String,
	pub tmp: String,

	pub use_pre_existing_cache: bool,
	pub use_pre_existing_db: bool
}

#[allow(deprecated)]
impl Config {
	pub fn new(fmt: &str) -> Result<Self, ConfigError> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();

		Ok(
			Self {
				root: format!("{}/{}/", home, fmt),
				cache: format!("{}/{}/cache/pkg_cache", home, fmt),
				rls: format!("{}/{}/cache/rls", home, fmt),
				tmp: format!("{}/{}/tmp", home, fmt),
				info: format!("{}/{}/info", home, fmt),
				use_pre_existing_cache: false,
				use_pre_existing_db: false,
			}
		)
	}

	pub fn from(file: &str) {
		let contents = fs::read_to_string(file).unwrap();
		let a: Self = serde_json::from_str(&contents).unwrap();
		println!("a = {:?}", a);
	}

	pub fn save(&self, to: &str) {
		let contents = serde_json::to_string(self).unwrap();
		fs::write(to, contents).unwrap();
	}

	pub fn setup(&self) -> Result<(), Error> {
		match fs::create_dir_all(&self.cache) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.rls) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.tmp) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.info) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		Ok(())
	}
}