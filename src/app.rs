use std::path::PathBuf;

use clap::{
	arg,
	crate_version,
	App,
	Arg,
};

pub struct Cmd {
	pub file_type: FileType,
	pub args: Vec<PathBuf>,
	pub recurse: bool,
	pub accessed: bool,
	pub modified: bool,
	pub created: bool,
	pub oldest: bool,
	pub hidden: bool,
	pub quiet: bool,
	pub n: usize,
}

impl Cmd {
	pub fn from_args() -> Self {
		let app = App::new("lf")
			.version(crate_version!())
			.about("Show most recent files.")
			.args(&[
				arg!(-a --accessed "Sort by date last accessed."),
				arg!(-m --modified "Sort by date last modified."),
				arg!(-c --created "Sort by date created."),
				arg!(-o --oldest "Show oldest files first."),
				arg!(-f --files "Show files, not directories."),
				arg!(-d --directories "Show directories, not files."),
				arg!(-D --hidden "Do not ignore hidden files and directories."),
				arg!(-r --recurse "Recursively search under directories."),
				arg!(-q --quiet "Do not report non fatal errors."),
				arg!(n: -n <N> "Show top N items.")
					.default_value("1")
					.validator(validate_positive_number),
				Arg::new("args")
					.help(
						"Any number of files or directories (glob patterns are parsed on windows).",
					)
					.default_value(".")
					.forbid_empty_values(true)
					.multiple_values(true),
			]);

		let m = app.get_matches_from(wild::args());

		let accessed = m.is_present("accessed");
		let mut modified = m.is_present("modified");
		let mut created = m.is_present("created");
		if !created && !modified && !accessed {
			created = true;
			modified = true;
		}

		let recurse = m.is_present("recurse");
		let hidden = m.is_present("hidden");
		let oldest = m.is_present("oldest");
		let quiet = m.is_present("quiet");

		let file_type = match (m.is_present("files"), m.is_present("directories")) {
			(true, false) => FileType::File,
			(false, true) => FileType::Directory,
			_ => FileType::Any,
		};

		let n = m.value_of("n").unwrap().parse::<usize>().unwrap();
		let args = m
			.values_of("args")
			.unwrap()
			.map(PathBuf::from)
			.collect::<Vec<_>>();

		Self {
			accessed,
			modified,
			created,
			quiet,
			recurse,
			hidden,
			n,
			args,
			oldest,
			file_type,
		}
	}
}

fn validate_positive_number(s: &str) -> Result<(), String> {
	match s.parse::<usize>() {
		Ok(0) | Err(_) => Err(String::from("the value must be a positive integer")),
		_ => Ok(()),
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FileType {
	Any,
	File,
	Directory,
}
