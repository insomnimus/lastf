use std::{
	error::Error,
	sync::mpsc,
	time::SystemTime,
};

use ignore::{
	WalkBuilder,
	WalkState::Continue,
};

use crate::app::{
	Cmd,
	FileType,
};

impl Cmd {
	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		let mut walker = WalkBuilder::new(&self.args[0]);
		for p in &self.args[1..] {
			walker.add(p);
		}

		let walker = walker
			.standard_filters(false)
			.hidden(self.hidden)
			.max_depth(if self.recurse { None } else { Some(1) })
			.build_parallel();

		let (tx, rx) = mpsc::channel();

		walker.run(move || {
			let tx = tx.clone();
			Box::new(move |res| {
				let (entry, md) = match res.and_then(|entry| entry.metadata().map(|md| (entry, md)))
				{
					Ok(e) => e,
					Err(e) => {
						if !self.quiet {
							eprintln!("error: {}", e);
						}
						return Continue;
					}
				};

				if !(md.file_type().is_dir() && self.file_type == FileType::File) {
					tx.send((entry, md)).unwrap();
				}
				Continue
			})
		});

		let mut vals: Vec<_> = rx
			.into_iter()
			.filter_map(|(entry, md)| {
				let _ = entry.path().file_name()?; // discard ./ etc
				let mut t = SystemTime::UNIX_EPOCH;
				if self.accessed {
					if let Ok(x) = md.accessed() {
						t = x;
					}
				}
				if self.modified {
					if let Ok(x) = md.modified() {
						t = t.max(x);
					}
				}
				if self.created {
					if let Ok(x) = md.created() {
						t = t.max(x);
					}
				}
				if t == SystemTime::UNIX_EPOCH {
					if !self.quiet {
						eprintln!(
							"warning: can't determine the age of {}",
							entry.path().display()
						);
					}
					None
				} else {
					Some((entry, t))
				}
			})
			.collect();

		vals.sort_by_key(|(_, t)| *t);

		if self.oldest {
			for (entry, _) in vals.iter().take(self.n) {
				println!("{}", entry.path().display());
			}
		} else {
			for (entry, _) in vals.iter().rev().take(self.n) {
				println!("{}", entry.path().display());
			}
		}
		Ok(())
	}
}
